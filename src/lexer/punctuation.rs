use crate::lexer::{
    assert_lex,
    heredoc_id::HeredocId,
    strings::types::{Heredoc, Interpolation, StringInterp, StringNoInterp, Symbol},
    Lexer, OnByte, StringLiteral,
};
use crate::token::{token, Token};

use crate::lexer::ident::Ident;
use crate::lexer::numbers::parse_number;
use crate::lexer::qmark::QMark;

impl<'a> OnByte<'a, b'#'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();

        // simply read until EOL
        loop {
            match self.buffer.current_byte() {
                None | Some(b'\n') => break,
                _ => self.buffer.skip_byte(),
            }
        }
        // Multiple consecutive comments are merged on the parser level

        token!(tCOMMENT, start, self.buffer.pos())
    }
}
assert_lex!(test_tCOMMENT_INLINE, b"# foo", tCOMMENT, b"# foo", 0..5);

impl<'a> OnByte<'a, b'*'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();

        match self.current_byte() {
            Some(b'*') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'=') => {
                        self.skip_byte();
                        token!(tOP_ASGN, start, start + 3)
                    }
                    _ => token!(tPOW, start, start + 2),
                }
            }
            Some(b'=') => {
                self.skip_byte();
                token!(tOP_ASGN, start, start + 2)
            }
            _ => token!(tSTAR, start, start + 1),
        }
    }
}
assert_lex!(test_tSTAR, b"*", tSTAR, b"*", 0..1);
assert_lex!(test_tOP_ASGN_STAR, b"*=", tOP_ASGN, b"*=", 0..2);
assert_lex!(test_tPOW, b"**", tPOW, b"**", 0..2);
assert_lex!(test_tOP_ASGN_DSTAR, b"**=", tOP_ASGN, b"**=", 0..3);

impl<'a> OnByte<'a, b'!'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();

        // !@ is handled on the parser level
        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                token!(tNEQ, start, start + 2)
            }
            Some(b'~') => {
                self.skip_byte();
                token!(tNMATCH, start, start + 2)
            }
            _ => token!(tBANG, start, start + 1),
        }
    }
}
assert_lex!(test_tNEQ, b"!=", tNEQ, b"!=", 0..2);
assert_lex!(test_tNMATCH, b"!~", tNMATCH, b"!~", 0..2);
assert_lex!(test_tBANG, b"!", tBANG, b"!", 0..1);

impl<'a> OnByte<'a, b'='> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();

        if self.buffer.lookahead(b"begin") {
            self.buffer.set_pos(self.pos() + 5);
            return token!(tEMBEDDED_COMMENT_START, start, start + 6);
        }

        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'=') => {
                        self.skip_byte();
                        token!(tEQQ, start, start + 3)
                    }
                    _ => token!(tEQ, start, start + 2),
                }
            }
            Some(b'~') => {
                self.skip_byte();
                token!(tMATCH, start, start + 2)
            }
            Some(b'>') => {
                self.skip_byte();
                token!(tASSOC, start, start + 2)
            }
            _ => token!(tEQL, start, start + 1),
        }
    }
}
assert_lex!(
    test_tEMBEDDED_COMMENT_START,
    b"=begin",
    tEMBEDDED_COMMENT_START,
    b"=begin",
    0..6
);
assert_lex!(test_tEQQ, b"===", tEQQ, b"===", 0..3);
assert_lex!(test_tEQ, b"==", tEQ, b"==", 0..2);
assert_lex!(test_tMATCH, b"=~", tMATCH, b"=~", 0..2);
assert_lex!(test_tASSOC, b"=>", tASSOC, b"=>", 0..2);
assert_lex!(test_tEQL, b"=", tEQL, b"=", 0..1);

impl<'a> OnByte<'a, b'<'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();

        // Check if heredoc id
        if let Some(b'<') = self.buffer.byte_at(start + 1) {
            if self.required_new_expr {
                if let Some(HeredocId {
                    token,
                    id: (id_start, id_end),
                    indent,
                    interp,
                }) = HeredocId::parse(&mut self.buffer)
                {
                    let interp = if interp {
                        Some(Interpolation::new(self.curly_nest))
                    } else {
                        None
                    };
                    self.string_literals
                        .push(StringLiteral::Heredoc(Heredoc::new(
                            interp,
                            self.buffer.slice(id_start, id_end),
                            id_end,
                            indent,
                        )));
                    return token;
                }
            }
        }

        self.skip_byte();

        // Otherwise just an operator
        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'>') => {
                        self.skip_byte();
                        token!(tCMP, start, start + 3)
                    }
                    _ => token!(tLEQ, start, start + 2),
                }
            }
            Some(b'<') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'=') => {
                        self.skip_byte();
                        token!(tOP_ASGN, start, start + 3)
                    }
                    _ => token!(tLSHFT, start, start + 2),
                }
            }
            _ => token!(tLT, start, start + 1),
        }
    }
}
assert_lex!(
    test_tSTRING_BEG_HEREDOC,
    b"<<-HERE",
    tDSTRING_BEG,
    b"<<-HERE",
    0..7,
    setup = |lexer: &mut Lexer| {
        lexer.curly_nest = 42;
        lexer.require_new_expr();
    },
    assert = |lexer: &Lexer| {
        assert_eq!(lexer.string_literals.size(), 1);

        assert_eq!(
            lexer.string_literals.last(),
            Some(StringLiteral::Heredoc(Heredoc::new(
                Some(Interpolation::new(42)),
                b"HERE",
                7,
                false
            )))
        );
    }
);
assert_lex!(test_tCMP, b"<=>", tCMP, b"<=>", 0..3);
assert_lex!(test_tLEQ, b"<=", tLEQ, b"<=", 0..2);
assert_lex!(test_tOP_ASGN_LSHIFT, b"<<=", tOP_ASGN, b"<<=", 0..3);
assert_lex!(test_tLSHFT, b"<<", tLSHFT, b"<<", 0..2);
assert_lex!(test_tLT, b"<", tLT, b"<", 0..1);

impl<'a> OnByte<'a, b'>'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                token!(tGEQ, start, start + 2)
            }
            Some(b'>') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'=') => {
                        self.skip_byte();
                        token!(tOP_ASGN, start, start + 3)
                    }
                    _ => token!(tRSHFT, start, start + 2),
                }
            }
            _ => token!(tGT, start, start + 1),
        }
    }
}
assert_lex!(test_tGEQ, b">=", tGEQ, b">=", 0..2);
assert_lex!(test_tOP_ASGN_RSHIFT, b">>=", tOP_ASGN, b">>=", 0..3);
assert_lex!(test_tRSHFT, b">>", tRSHFT, b">>", 0..2);
assert_lex!(test_tGT, b">", tGT, b">", 0..1);

impl<'a> OnByte<'a, b'"'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        let token = token!(tDSTRING_BEG, start, start + 1);
        self.string_literals
            .push(StringLiteral::StringInterp(StringInterp::new(
                Interpolation::new(self.curly_nest),
                b'"',
            )));
        token
    }
}
assert_lex!(
    test_tSTRING_BEG_DQUOTE,
    b"\"",
    tDSTRING_BEG,
    b"\"",
    0..1,
    setup = |lexer: &mut Lexer| {
        lexer.curly_nest = 42;
    },
    assert = |lexer: &Lexer| {
        use crate::lexer::strings::types::StringInterp;
        assert_eq!(lexer.string_literals.size(), 1);

        assert_eq!(
            lexer.string_literals.last(),
            Some(StringLiteral::StringInterp(StringInterp::new(
                Interpolation::new(42),
                b'"'
            )))
        );
    }
);

impl<'a> OnByte<'a, b'`'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        todo!("unclear what to do here?? MRI does state-dependent analysis")
    }
}

impl<'a> OnByte<'a, b'\''> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        let token = token!(tSTRING_BEG, start, start + 1);
        self.string_literals
            .push(StringLiteral::StringNoInterp(StringNoInterp::new(b'\'')));
        token
    }
}
assert_lex!(
    test_tSTRING_BEG1_SQUOTE,
    b"'",
    tSTRING_BEG,
    b"'",
    0..1,
    setup = |lexer: &mut Lexer| {
        lexer.curly_nest = 42;
    },
    assert = |lexer: &Lexer| {
        assert_eq!(lexer.string_literals.size(), 1);
        assert_eq!(
            lexer.string_literals.last(),
            Some(StringLiteral::StringNoInterp(StringNoInterp::new(b'\'')))
        )
    }
);

impl<'a> OnByte<'a, b'?'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        QMark::parse(&mut self.buffer)
    }
}

impl<'a> OnByte<'a, b'&'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        match self.current_byte() {
            Some(b'&') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'=') => {
                        self.skip_byte();
                        token!(tOP_ASGN, start, start + 3)
                    }
                    _ => token!(tANDOP, start, start + 2),
                }
            }
            Some(b'=') => {
                self.skip_byte();
                token!(tOP_ASGN, start, start + 2)
            }
            Some(b'.') => {
                self.skip_byte();
                token!(tANDDOT, start, start + 2)
            }
            _ => token!(tAMPER, start, start + 1),
        }
    }
}
assert_lex!(test_tOP_ASGN_DAMPER, b"&&=", tOP_ASGN, b"&&=", 0..3);
assert_lex!(test_tANDOP, b"&&", tANDOP, b"&&", 0..2);
assert_lex!(test_tOP_ASGN_AMPER, b"&=", tOP_ASGN, b"&=", 0..2);
assert_lex!(test_tANDDOT, b"&.", tANDDOT, b"&.", 0..2);
assert_lex!(test_tAMPER, b"&", tAMPER, b"&", 0..1);

impl<'a> OnByte<'a, b'|'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        match self.current_byte() {
            Some(b'|') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'=') => {
                        self.skip_byte();
                        token!(tOP_ASGN, start, start + 3)
                    }
                    _ => token!(tOROP, start, start + 2),
                }
            }
            Some(b'=') => {
                self.skip_byte();
                token!(tOP_ASGN, start, start + 2)
            }
            _ => token!(tPIPE, start, start + 1),
        }
    }
}
assert_lex!(test_tOP_ASGN_DPIPE, b"||=", tOP_ASGN, b"||=", 0..3);
assert_lex!(test_tOROP, b"||", tOROP, b"||", 0..2);
assert_lex!(test_tOP_ASGN_PIPE, b"|=", tOP_ASGN, b"|=", 0..2);
assert_lex!(test_tPIPE, b"|", tPIPE, b"|", 0..1);

impl<'a> OnByte<'a, b'+'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        // +@ is handled on the parser level
        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                token!(tOP_ASGN, start, start + 2)
            }
            Some(b'0'..=b'9') => {
                let mut token = parse_number(&mut self.buffer);
                token.1 .0 = start;
                token
            }
            _ => token!(tPLUS, start, start + 1),
        }
    }
}
assert_lex!(test_tOP_ASGN_PLUS, b"+=", tOP_ASGN, b"+=", 0..2);
assert_lex!(test_tPLUS_NUMBER, b"+1", tINTEGER, b"+1", 0..2);
assert_lex!(test_tPLUS, b"+", tPLUS, b"+", 0..1);

impl<'a> OnByte<'a, b'-'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        // -@ is handled on the parser level
        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                token!(tOP_ASGN, start, start + 2)
            }
            Some(b'>') => {
                self.skip_byte();
                token!(tLAMBDA, start, start + 2)
            }
            Some(b'0'..=b'9') => token!(tUMINUS, start, start + 1),
            _ => token!(tMINUS, start, start + 1),
        }
    }
}
assert_lex!(test_tOP_ASGN_MINUS, b"-=", tOP_ASGN, b"-=", 0..2);
assert_lex!(test_tLAMBDA, b"->", tLAMBDA, b"->", 0..2);
assert_lex!(test_tMINUS, b"-", tMINUS, b"-", 0..1);
assert_lex!(test_tUMINUS, b"-5", tUMINUS, b"-", 0..1);

impl<'a> OnByte<'a, b'.'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        match self.current_byte() {
            Some(b'.') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'.') => {
                        self.skip_byte();
                        token!(tDOT3, start, start + 3)
                    }
                    _ => token!(tDOT2, start, start + 2),
                }
            }
            Some(b'0'..=b'9') => {
                todo!("Handle .<n> case as error?? Skip all number until NaN found");
            }
            _ => token!(tDOT, start, start + 1),
        }
    }
}
assert_lex!(test_tDOT3, b"...", tDOT3, b"...", 0..3);
assert_lex!(test_tDOT2, b"..", tDOT2, b"..", 0..2);
assert_lex!(test_tDOT, b".", tDOT, b".", 0..1);

impl<'a> OnByte<'a, b')'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        if self.paren_nest > 0 {
            self.paren_nest -= 1;
        } else {
            panic!("negative paren_nest");
        }

        token!(tRPAREN, start, start + 1)
    }
}
assert_lex!(
    test_tRPAREN,
    b")",
    tRPAREN,
    b")",
    0..1,
    setup = |lexer: &mut Lexer| {
        lexer.paren_nest = 1;
    },
    assert = |_lexer: &Lexer| {}
);

impl<'a> OnByte<'a, b']'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        if self.brack_nest > 0 {
            self.brack_nest -= 1;
        } else {
            panic!("negative brack_nest");
        }
        token!(tRBRACK, start, start + 1)
    }
}
assert_lex!(
    test_tRBRACK,
    b"]",
    tRBRACK,
    b"]",
    0..1,
    setup = |lexer: &mut Lexer| {
        lexer.brack_nest = 1;
    },
    assert = |_lexer: &Lexer| {}
);

impl<'a> OnByte<'a, b'}'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        if self.curly_nest > 0 {
            self.curly_nest -= 1;
        } else {
            panic!("negative curly_nest");
        }
        token!(tRCURLY, start, start + 1)
    }
}
assert_lex!(
    test_tRCURLY,
    b"}",
    tRCURLY,
    b"}",
    0..1,
    setup = |lexer: &mut Lexer| {
        lexer.curly_nest = 1;
    },
    assert = |_lexer: &Lexer| {}
);

impl<'a> OnByte<'a, b':'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        match self.current_byte() {
            Some(b':') => {
                self.skip_byte();
                token!(tCOLON2, start, start + 2)
            }
            Some(b'"') => {
                // :"..." symbol
                self.skip_byte();
                let token = token!(tDSYMBEG, start, start + 2);
                self.string_literals
                    .push(StringLiteral::Symbol(Symbol::new(true, self.curly_nest)));
                token
            }
            Some(b'\'') => {
                // :'...' symbol
                self.skip_byte();
                let token = token!(tSYMBEG, start, start + 2);
                self.string_literals
                    .push(StringLiteral::Symbol(Symbol::new(false, self.curly_nest)));
                token
            }
            _ => token!(tCOLON, start, start + 1),
        }
    }
}
assert_lex!(test_tCOLON2, b"::", tCOLON2, b"::", 0..2);
assert_lex!(
    test_tDSYMBEG,
    b":\"",
    tDSYMBEG,
    b":\"",
    0..2,
    setup = |lexer: &mut Lexer| {
        lexer.curly_nest = 42;
    },
    assert = |lexer: &Lexer| {
        use crate::lexer::strings::types::Symbol;

        assert_eq!(lexer.string_literals.size(), 1);
        assert_eq!(
            lexer.string_literals.last(),
            Some(StringLiteral::Symbol(Symbol::new(true, 42)))
        )
    }
);
assert_lex!(
    test_tSYMBEG,
    b":'",
    tSYMBEG,
    b":'",
    0..2,
    setup = |lexer: &mut Lexer| {
        lexer.curly_nest = 42;
    },
    assert = |lexer: &Lexer| {
        use crate::lexer::strings::types::Symbol;

        assert_eq!(lexer.string_literals.size(), 1);
        assert_eq!(
            lexer.string_literals.last(),
            Some(StringLiteral::Symbol(Symbol::new(false, 42)))
        )
    }
);
assert_lex!(test_tCOLON, b":", tCOLON, b":", 0..1);

impl<'a> OnByte<'a, b'/'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        // Regexp begin is handled on the parser level

        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                token!(tOP_ASGN, start, start + 2)
            }
            _ => token!(tDIVIDE, start, start + 1),
        }
    }
}
assert_lex!(test_tOP_ASGN_DIV, b"/=", tOP_ASGN, b"/=", 0..2);
assert_lex!(test_tDIVIDE, b"/", tDIVIDE, b"/", 0..1);

impl<'a> OnByte<'a, b'^'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();

        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                token!(tOP_ASGN, start, start + 2)
            }
            _ => token!(tCARET, start, start + 1),
        }
    }
}
assert_lex!(test_tOP_ASGN_CARET, b"^=", tOP_ASGN, b"^=", 0..2);
assert_lex!(test_tCARET, b"^", tCARET, b"^", 0..1);

impl<'a> OnByte<'a, b';'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        token!(tSEMI, start, start + 1)
    }
}
assert_lex!(test_tSEMI, b";", tSEMI, b";", 0..1);

impl<'a> OnByte<'a, b','> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        token!(tCOMMA, start, start + 1)
    }
}
assert_lex!(test_tCOMMA, b",", tCOMMA, b",", 0..1);

impl<'a> OnByte<'a, b'~'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        // ~@ is handled on the parser level
        token!(tTILDE, start, start + 1)
    }
}
assert_lex!(test_tTILDE, b"~", tTILDE, b"~", 0..1);

impl<'a> OnByte<'a, b'('> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        self.paren_nest += 1;
        token!(tLPAREN, start, start + 1)
    }
}
assert_lex!(test_tLPAREN, b"(", tLPAREN, b"(", 0..1);

impl<'a> OnByte<'a, b'['> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        self.brack_nest += 1;
        token!(tLBRACK, start, start + 1)
    }
}
assert_lex!(test_tLBRACK, b"[", tLBRACK, b"[", 0..1);

impl<'a> OnByte<'a, b'{'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        self.curly_nest += 1;
        token!(tLCURLY, start, start + 1)
    }
}

impl<'a> OnByte<'a, b'\\'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        match self.current_byte() {
            Some(b'\n') => {
                self.skip_byte();
                return self.next_token();
            }
            Some(b' ') => {
                self.skip_byte();
                token!(tSP, start, start + 2)
            }
            Some(b'\t') => {
                self.skip_byte();
                token!(tSLASH_T, start, start + 2)
            }
            Some(0x0c) => {
                self.skip_byte();
                token!(tSLASH_F, start, start + 2)
            }
            Some(b'\r') => {
                self.skip_byte();
                token!(tSLASH_R, start, start + 2)
            }
            Some(0x0b) => {
                self.skip_byte();
                token!(tVTAB, start, start + 2)
            }
            _ => token!(tBACKSLASH, start, start + 1),
        }
    }
}
assert_lex!(
    test_tESCAPED_NL,
    b"\\\nTEST_TOKEN",
    tTEST_TOKEN,
    b"TEST_TOKEN",
    2..12
);
assert_lex!(test_tESCAPED_SP, b"\\ ", tSP, b"\\ ", 0..2);
assert_lex!(test_tESCAPED_TAB, b"\\\t", tSLASH_T, b"\\\t", 0..2);
assert_lex!(test_tESCAPED_LF, b"\\\x0c", tSLASH_F, b"\\\x0c", 0..2);
assert_lex!(test_tESCAPED_CR, b"\\\r", tSLASH_R, b"\\\r", 0..2);
assert_lex!(test_tESCAPED_VTAB, b"\\\x0b", tVTAB, b"\\\x0b", 0..2);

impl<'a> OnByte<'a, b'_'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();

        match start
            .checked_sub(1)
            .map(|idx| self.buffer.byte_at(idx))
            .flatten()
        {
            // prev byte is either
            //   + None (i.e. it's the first byte of the file)
            //   + Some(b'\n')
            // AND it's "__END__" sequence
            None | Some(b'\n') if self.buffer.lookahead(b"__END__") => {
                return token!(tEOF, start, start);
            }
            _ => {}
        }

        self.buffer.set_pos(start);
        Ident::parse(&mut self.buffer)
    }
}
assert_lex!(test_tEOF_at__END__, b"__END__", tEOF, b"", 0..0);
assert_lex!(test_tEOF_at_NL___END__, b"\n__END__", tEOF, b"", 1..1);
