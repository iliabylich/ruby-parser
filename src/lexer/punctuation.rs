use crate::{
    lexer::{
        assert_lex,
        heredoc_id::HeredocId,
        ident::Ident,
        numbers::parse_number,
        qmark::QMark,
        strings::types::{Heredoc, Interpolation, StringInterp, StringPlain, Symbol},
        Lexer, OnByte, StringLiteral,
    },
    loc::loc,
    token::{token, Token},
};

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

        token!(tCOMMENT, loc!(start, self.buffer.pos()))
    }
}
assert_lex!(test_tCOMMENT_INLINE, b"# foo", tCOMMENT, None, 0..5);

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
                        token!(tOP_ASGN, loc!(start, start + 3))
                    }
                    _ => token!(tPOW, loc!(start, start + 2)),
                }
            }
            Some(b'=') => {
                self.skip_byte();
                token!(tOP_ASGN, loc!(start, start + 2))
            }
            _ => token!(tSTAR, loc!(start, start + 1)),
        }
    }
}
assert_lex!(test_tSTAR, b"*", tSTAR, None, 0..1);
assert_lex!(test_tOP_ASGN_STAR, b"*=", tOP_ASGN, None, 0..2);
assert_lex!(test_tPOW, b"**", tPOW, None, 0..2);
assert_lex!(test_tOP_ASGN_DSTAR, b"**=", tOP_ASGN, None, 0..3);

impl<'a> OnByte<'a, b'!'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();

        // !@ is handled on the parser level
        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                token!(tNEQ, loc!(start, start + 2))
            }
            Some(b'~') => {
                self.skip_byte();
                token!(tNMATCH, loc!(start, start + 2))
            }
            _ => token!(tBANG, loc!(start, start + 1)),
        }
    }
}
assert_lex!(test_tNEQ, b"!=", tNEQ, None, 0..2);
assert_lex!(test_tNMATCH, b"!~", tNMATCH, None, 0..2);
assert_lex!(test_tBANG, b"!", tBANG, None, 0..1);

impl<'a> OnByte<'a, b'='> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();

        if self.buffer.lookahead(b"begin") {
            self.buffer.set_pos(self.pos() + 5);
            return token!(tEMBEDDED_COMMENT_START, loc!(start, start + 6));
        }

        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'=') => {
                        self.skip_byte();
                        token!(tEQQ, loc!(start, start + 3))
                    }
                    _ => token!(tEQ, loc!(start, start + 2)),
                }
            }
            Some(b'~') => {
                self.skip_byte();
                token!(tMATCH, loc!(start, start + 2))
            }
            Some(b'>') => {
                self.skip_byte();
                token!(tASSOC, loc!(start, start + 2))
            }
            _ => token!(tEQL, loc!(start, start + 1)),
        }
    }
}
assert_lex!(
    test_tEMBEDDED_COMMENT_START,
    b"=begin",
    tEMBEDDED_COMMENT_START,
    None,
    0..6
);
assert_lex!(test_tEQQ, b"===", tEQQ, None, 0..3);
assert_lex!(test_tEQ, b"==", tEQ, None, 0..2);
assert_lex!(test_tMATCH, b"=~", tMATCH, None, 0..2);
assert_lex!(test_tASSOC, b"=>", tASSOC, None, 0..2);
assert_lex!(test_tEQL, b"=", tEQL, None, 0..1);

impl<'a> OnByte<'a, b'<'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();

        // Check if heredoc id
        if let Some(b'<') = self.buffer.byte_at(start + 1) {
            if self.required_new_expr {
                if let Some(HeredocId {
                    token,
                    id: (id_start, id_end),
                    squiggly,
                    interpolated,
                }) = HeredocId::parse(&mut self.buffer)
                {
                    let interpolated = if interpolated {
                        Some(Interpolation::new(self.curly_nest))
                    } else {
                        None
                    };
                    self.string_literals
                        .push(StringLiteral::Heredoc(Heredoc::new(
                            interpolated,
                            self.buffer.slice(id_start, id_end).expect("bug"),
                            id_end,
                            squiggly,
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
                        token!(tCMP, loc!(start, start + 3))
                    }
                    _ => token!(tLEQ, loc!(start, start + 2)),
                }
            }
            Some(b'<') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'=') => {
                        self.skip_byte();
                        token!(tOP_ASGN, loc!(start, start + 3))
                    }
                    _ => token!(tLSHFT, loc!(start, start + 2)),
                }
            }
            _ => token!(tLT, loc!(start, start + 1)),
        }
    }
}
assert_lex!(
    test_name = test_tSTRING_BEG_HEREDOC,
    input = b"<<-HERE",
    kind = tDSTRING_BEG,
    value = None,
    loc = 0..7,
    setup = |lexer: &mut Lexer| {
        lexer.curly_nest = 42;
        lexer.require_new_expr();
    },
    assert = |lexer: &Lexer| {
        assert_eq!(lexer.string_literals.size(), 1);

        assert_eq!(
            lexer.string_literals.last(),
            Some(&StringLiteral::Heredoc(Heredoc::new(
                Some(Interpolation::new(42)),
                b"HERE",
                7,
                false
            )))
        );
    }
);
assert_lex!(test_tCMP, b"<=>", tCMP, None, 0..3);
assert_lex!(test_tLEQ, b"<=", tLEQ, None, 0..2);
assert_lex!(test_tOP_ASGN_LSHIFT, b"<<=", tOP_ASGN, None, 0..3);
assert_lex!(test_tLSHFT, b"<<", tLSHFT, None, 0..2);
assert_lex!(test_tLT, b"<", tLT, None, 0..1);

impl<'a> OnByte<'a, b'>'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                token!(tGEQ, loc!(start, start + 2))
            }
            Some(b'>') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'=') => {
                        self.skip_byte();
                        token!(tOP_ASGN, loc!(start, start + 3))
                    }
                    _ => token!(tRSHFT, loc!(start, start + 2)),
                }
            }
            _ => token!(tGT, loc!(start, start + 1)),
        }
    }
}
assert_lex!(test_tGEQ, b">=", tGEQ, None, 0..2);
assert_lex!(test_tOP_ASGN_RSHIFT, b">>=", tOP_ASGN, None, 0..3);
assert_lex!(test_tRSHFT, b">>", tRSHFT, None, 0..2);
assert_lex!(test_tGT, b">", tGT, None, 0..1);

impl<'a> OnByte<'a, b'"'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        let token = token!(tDSTRING_BEG, loc!(start, start + 1));
        self.string_literals
            .push(StringLiteral::StringInterp(StringInterp::new(
                Interpolation::new(self.curly_nest),
                b'"',
                b'"',
            )));
        token
    }
}
assert_lex!(
    test_name = test_tSTRING_BEG_DQUOTE,
    input = b"\"",
    kind = tDSTRING_BEG,
    value = None,
    loc = 0..1,
    setup = |lexer: &mut Lexer| {
        lexer.curly_nest = 42;
    },
    assert = |lexer: &Lexer| {
        use crate::lexer::strings::types::StringInterp;
        assert_eq!(lexer.string_literals.size(), 1);

        assert_eq!(
            lexer.string_literals.last(),
            Some(&StringLiteral::StringInterp(StringInterp::new(
                Interpolation::new(42),
                b'"',
                b'"'
            )))
        );
    }
);

impl<'a> OnByte<'a, b'`'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        // we rewrite '`' to tXSTRING_BEG on the parser level
        let start = self.pos();
        self.skip_byte();
        token!(tIDENTIFIER, loc!(start, start + 1))
    }
}
assert_lex!(test_tIDENTIFIER_backtick, b"`", tIDENTIFIER, None, 0..1);

impl<'a> OnByte<'a, b'\''> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        let token = token!(tSTRING_BEG, loc!(start, start + 1));
        self.string_literals
            .push(StringLiteral::StringPlain(StringPlain::new(b'\'', b'\'')));
        token
    }
}
assert_lex!(
    test_name = test_tSTRING_BEG1_SQUOTE,
    input = b"'",
    kind = tSTRING_BEG,
    value = None,
    loc = 0..1,
    setup = |lexer: &mut Lexer| {
        lexer.curly_nest = 42;
    },
    assert = |lexer: &Lexer| {
        assert_eq!(lexer.string_literals.size(), 1);
        assert_eq!(
            lexer.string_literals.last(),
            Some(&StringLiteral::StringPlain(StringPlain::new(b'\'', b'\'')))
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
                        token!(tOP_ASGN, loc!(start, start + 3))
                    }
                    _ => token!(tANDOP, loc!(start, start + 2)),
                }
            }
            Some(b'=') => {
                self.skip_byte();
                token!(tOP_ASGN, loc!(start, start + 2))
            }
            Some(b'.') => {
                self.skip_byte();
                token!(tANDDOT, loc!(start, start + 2))
            }
            _ => token!(tAMPER, loc!(start, start + 1)),
        }
    }
}
assert_lex!(test_tOP_ASGN_DAMPER, b"&&=", tOP_ASGN, None, 0..3);
assert_lex!(test_tANDOP, b"&&", tANDOP, None, 0..2);
assert_lex!(test_tOP_ASGN_AMPER, b"&=", tOP_ASGN, None, 0..2);
assert_lex!(test_tANDDOT, b"&.", tANDDOT, None, 0..2);
assert_lex!(test_tAMPER, b"&", tAMPER, None, 0..1);

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
                        token!(tOP_ASGN, loc!(start, start + 3))
                    }
                    _ => token!(tOROP, loc!(start, start + 2)),
                }
            }
            Some(b'=') => {
                self.skip_byte();
                token!(tOP_ASGN, loc!(start, start + 2))
            }
            _ => token!(tPIPE, loc!(start, start + 1)),
        }
    }
}
assert_lex!(test_tOP_ASGN_DPIPE, b"||=", tOP_ASGN, None, 0..3);
assert_lex!(test_tOROP, b"||", tOROP, None, 0..2);
assert_lex!(test_tOP_ASGN_PIPE, b"|=", tOP_ASGN, None, 0..2);
assert_lex!(test_tPIPE, b"|", tPIPE, None, 0..1);

impl<'a> OnByte<'a, b'+'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        // +@ is handled on the parser level
        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                token!(tOP_ASGN, loc!(start, start + 2))
            }
            Some(b'0'..=b'9') => {
                let mut token = parse_number(&mut self.buffer);
                token.loc.start = start;
                token
            }
            _ => token!(tPLUS, loc!(start, start + 1)),
        }
    }
}
assert_lex!(test_tOP_ASGN_PLUS, b"+=", tOP_ASGN, None, 0..2);
assert_lex!(test_tPLUS_NUMBER, b"+1", tINTEGER, None, 0..2);
assert_lex!(test_tPLUS, b"+", tPLUS, None, 0..1);

impl<'a> OnByte<'a, b'-'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        // -@ is handled on the parser level
        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                token!(tOP_ASGN, loc!(start, start + 2))
            }
            Some(b'>') => {
                self.skip_byte();
                token!(tLAMBDA, loc!(start, start + 2))
            }
            Some(b'0'..=b'9') => token!(tUMINUS, loc!(start, start + 1)),
            _ => token!(tMINUS, loc!(start, start + 1)),
        }
    }
}
assert_lex!(test_tOP_ASGN_MINUS, b"-=", tOP_ASGN, None, 0..2);
assert_lex!(test_tLAMBDA, b"->", tLAMBDA, None, 0..2);
assert_lex!(test_tMINUS, b"-", tMINUS, None, 0..1);
assert_lex!(test_tUMINUS, b"-5", tUMINUS, None, 0..1);

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
                        token!(tDOT3, loc!(start, start + 3))
                    }
                    _ => token!(tDOT2, loc!(start, start + 2)),
                }
            }
            Some(b'0'..=b'9') => {
                let mut end = start;
                while matches!(self.buffer.byte_at(end), Some(b'0'..=b'9')) {
                    end += 1;
                }
                panic!(
                    "no .<digit> floating literal anymore; put 0 before dot ({:?})",
                    self.buffer.slice(start, end)
                );
            }
            _ => token!(tDOT, loc!(start, start + 1)),
        }
    }
}
assert_lex!(test_tDOT3, b"...", tDOT3, None, 0..3);
assert_lex!(test_tDOT2, b"..", tDOT2, None, 0..2);
assert_lex!(test_tDOT, b".", tDOT, None, 0..1);

impl<'a> OnByte<'a, b')'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        if self.paren_nest > 0 {
            self.paren_nest -= 1;
        } else {
            panic!("negative paren_nest");
        }

        token!(tRPAREN, loc!(start, start + 1))
    }
}
assert_lex!(
    test_name = test_tRPAREN,
    input = b")",
    kind = tRPAREN,
    value = None,
    loc = 0..1,
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
        token!(tRBRACK, loc!(start, start + 1))
    }
}
assert_lex!(
    test_name = test_tRBRACK,
    input = b"]",
    kind = tRBRACK,
    value = None,
    loc = 0..1,
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
        token!(tRCURLY, loc!(start, start + 1))
    }
}
assert_lex!(
    test_name = test_tRCURLY,
    input = b"}",
    kind = tRCURLY,
    value = None,
    loc = 0..1,
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
                return token!(tCOLON2, loc!(start, start + 2));
            }
            Some(b'"') => {
                // :"..." symbol
                self.skip_byte();
                let token = token!(tDSYMBEG, loc!(start, start + 2));
                self.string_literals
                    .push(StringLiteral::Symbol(Symbol::new(true, self.curly_nest)));
                return token;
            }
            Some(b'\'') => {
                // :'...' symbol
                self.skip_byte();
                let token = token!(tSYMBEG, loc!(start, start + 2));
                self.string_literals
                    .push(StringLiteral::Symbol(Symbol::new(false, self.curly_nest)));
                return token;
            }
            _ => {}
        }

        // Plain symbols are handled on the parser level
        // by concatenating tCOLON and tIDENTIFIER/tCONSTANT/tIVAR/tCVAR/tGVAR

        token!(tCOLON, loc!(start, start + 1))
    }
}
assert_lex!(test_tCOLON2, b"::", tCOLON2, None, 0..2);
assert_lex!(
    test_name = test_tDSYMBEG,
    input = b":\"",
    kind = tDSYMBEG,
    value = None,
    loc = 0..2,
    setup = |lexer: &mut Lexer| {
        lexer.curly_nest = 42;
    },
    assert = |lexer: &Lexer| {
        use crate::lexer::strings::types::Symbol;

        assert_eq!(lexer.string_literals.size(), 1);
        assert_eq!(
            lexer.string_literals.last(),
            Some(&StringLiteral::Symbol(Symbol::new(true, 42)))
        )
    }
);
assert_lex!(
    test_name = test_tSYMBEG,
    input = b":'",
    kind = tSYMBEG,
    value = None,
    loc = 0..2,
    setup = |lexer: &mut Lexer| {
        lexer.curly_nest = 42;
    },
    assert = |lexer: &Lexer| {
        use crate::lexer::strings::types::Symbol;

        assert_eq!(lexer.string_literals.size(), 1);
        assert_eq!(
            lexer.string_literals.last(),
            Some(&StringLiteral::Symbol(Symbol::new(false, 42)))
        )
    }
);
assert_lex!(test_tCOLON, b":", tCOLON, None, 0..1);

impl<'a> OnByte<'a, b'/'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        // Regexp begin is handled on the parser level

        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                token!(tOP_ASGN, loc!(start, start + 2))
            }
            _ => token!(tDIVIDE, loc!(start, start + 1)),
        }
    }
}
assert_lex!(test_tOP_ASGN_DIV, b"/=", tOP_ASGN, None, 0..2);
assert_lex!(test_tDIVIDE, b"/", tDIVIDE, None, 0..1);

impl<'a> OnByte<'a, b'^'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();

        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                token!(tOP_ASGN, loc!(start, start + 2))
            }
            _ => token!(tCARET, loc!(start, start + 1)),
        }
    }
}
assert_lex!(test_tOP_ASGN_CARET, b"^=", tOP_ASGN, None, 0..2);
assert_lex!(test_tCARET, b"^", tCARET, None, 0..1);

impl<'a> OnByte<'a, b';'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        token!(tSEMI, loc!(start, start + 1))
    }
}
assert_lex!(test_tSEMI, b";", tSEMI, None, 0..1);

impl<'a> OnByte<'a, b','> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        token!(tCOMMA, loc!(start, start + 1))
    }
}
assert_lex!(test_tCOMMA, b",", tCOMMA, None, 0..1);

impl<'a> OnByte<'a, b'~'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        // ~@ is handled on the parser level
        token!(tTILDE, loc!(start, start + 1))
    }
}
assert_lex!(test_tTILDE, b"~", tTILDE, None, 0..1);

impl<'a> OnByte<'a, b'('> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        self.paren_nest += 1;
        token!(tLPAREN, loc!(start, start + 1))
    }
}
assert_lex!(test_tLPAREN, b"(", tLPAREN, None, 0..1);

impl<'a> OnByte<'a, b'['> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        self.brack_nest += 1;
        token!(tLBRACK, loc!(start, start + 1))
    }
}
assert_lex!(test_tLBRACK, b"[", tLBRACK, None, 0..1);

impl<'a> OnByte<'a, b'{'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        self.curly_nest += 1;
        token!(tLCURLY, loc!(start, start + 1))
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
                token!(tSP, loc!(start, start + 2))
            }
            Some(b'\t') => {
                self.skip_byte();
                token!(tSLASH_T, loc!(start, start + 2))
            }
            Some(0x0c) => {
                self.skip_byte();
                token!(tSLASH_F, loc!(start, start + 2))
            }
            Some(b'\r') => {
                self.skip_byte();
                token!(tSLASH_R, loc!(start, start + 2))
            }
            Some(0x0b) => {
                self.skip_byte();
                token!(tVTAB, loc!(start, start + 2))
            }
            _ => token!(tBACKSLASH, loc!(start, start + 1)),
        }
    }
}
assert_lex!(
    test_tESCAPED_NL,
    b"\\\nTEST_TOKEN",
    tTEST_TOKEN,
    None,
    2..12
);
assert_lex!(test_tESCAPED_SP, b"\\ ", tSP, None, 0..2);
assert_lex!(test_tESCAPED_TAB, b"\\\t", tSLASH_T, None, 0..2);
assert_lex!(test_tESCAPED_LF, b"\\\x0c", tSLASH_F, None, 0..2);
assert_lex!(test_tESCAPED_CR, b"\\\r", tSLASH_R, None, 0..2);
assert_lex!(test_tESCAPED_VTAB, b"\\\x0b", tVTAB, None, 0..2);

impl<'a> OnByte<'a, b'_'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();

        let prev_byte = start
            .checked_sub(1)
            .map(|idx| self.buffer.byte_at(idx))
            .flatten();
        match prev_byte {
            // prev byte is either
            //   + None (i.e. it's the first byte of the file)
            //   + Some(b'\n')
            // AND it's "__END__" sequence
            None | Some(b'\n') if self.buffer.lookahead(b"__END__") => {
                return token!(tEOF, loc!(start, start));
            }
            _ => {}
        }

        // otherwise it's a `_foo`/`_foo?`/`_foo!` identifier
        Ident::parse(&mut self.buffer)
    }
}
assert_lex!(test_tEOF_at__END__, b"__END__", tEOF, None, 0..0);
assert_lex!(test_tEOF_at_NL___END__, b"\n__END__", tEOF, None, 1..1);
assert_lex!(test_underscore_ident, b"_foo", tIDENTIFIER, None, 0..4);
assert_lex!(test_underscore_fid, b"_foo?", tFID, None, 0..5);
