use crate::{
    lexer::{
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

impl OnByte<b'#'> for Lexer {
    fn on_byte(&mut self) -> Token {
        let start = self.pos();

        // simply read until EOL
        loop {
            match self.buffer().current_byte() {
                None | Some(b'\n') => break,
                _ => self.buffer().skip_byte(),
            }
        }
        // Multiple consecutive comments are merged on the parser level

        token!(tCOMMENT, loc!(start, self.buffer().pos()))
    }
}
#[test]
fn test_tCOMMENT_INLINE() {
    use crate::testing::assert_lex;
    assert_lex!(b"# foo", token!(tCOMMENT, loc!(0, 5)));
}

impl OnByte<b'*'> for Lexer {
    fn on_byte(&mut self) -> Token {
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
#[test]
fn test_tSTAR() {
    use crate::testing::assert_lex;
    assert_lex!(b"*", token!(tSTAR, loc!(0, 1)));
}
#[test]
fn test_tOP_ASGN_STAR() {
    use crate::testing::assert_lex;
    assert_lex!(b"*=", token!(tOP_ASGN, loc!(0, 2)));
}
#[test]
fn test_tPOW() {
    use crate::testing::assert_lex;
    assert_lex!(b"**", token!(tPOW, loc!(0, 2)));
}
#[test]
fn test_tOP_ASGN_DSTAR() {
    use crate::testing::assert_lex;
    assert_lex!(b"**=", token!(tOP_ASGN, loc!(0, 3)));
}

impl OnByte<b'!'> for Lexer {
    fn on_byte(&mut self) -> Token {
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
#[test]
fn test_tNEQ() {
    use crate::testing::assert_lex;
    assert_lex!(b"!=", token!(tNEQ, loc!(0, 2)));
}
#[test]
fn test_tNMATCH() {
    use crate::testing::assert_lex;
    assert_lex!(b"!~", token!(tNMATCH, loc!(0, 2)));
}
#[test]
fn test_tBANG() {
    use crate::testing::assert_lex;
    assert_lex!(b"!", token!(tBANG, loc!(0, 1)));
}

impl OnByte<b'='> for Lexer {
    fn on_byte(&mut self) -> Token {
        let start = self.pos();
        self.skip_byte();

        if self.buffer().lookahead(b"begin") {
            self.buffer().set_pos(self.pos() + 5);
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
#[test]
fn test_tEMBEDDED_COMMENT_START() {
    use crate::testing::assert_lex;
    assert_lex!(b"=begin", token!(tEMBEDDED_COMMENT_START, loc!(0, 6)));
}
#[test]
fn test_tEQQ() {
    use crate::testing::assert_lex;
    assert_lex!(b"===", token!(tEQQ, loc!(0, 3)));
}
#[test]
fn test_tEQ() {
    use crate::testing::assert_lex;
    assert_lex!(b"==", token!(tEQ, loc!(0, 2)));
}
#[test]
fn test_tMATCH() {
    use crate::testing::assert_lex;
    assert_lex!(b"=~", token!(tMATCH, loc!(0, 2)));
}
#[test]
fn test_tASSOC() {
    use crate::testing::assert_lex;
    assert_lex!(b"=>", token!(tASSOC, loc!(0, 2)));
}
#[test]
fn test_tEQL() {
    use crate::testing::assert_lex;
    assert_lex!(b"=", token!(tEQL, loc!(0, 1)));
}

impl OnByte<b'<'> for Lexer {
    fn on_byte(&mut self) -> Token {
        let start = self.pos();

        // Check if heredoc id
        if let Some(b'<') = self.buffer().byte_at(start + 1) {
            if self.required_new_expr() {
                if let Some(HeredocId {
                    token,
                    id: (id_start, id_end),
                    squiggly,
                    interpolated,
                }) = HeredocId::parse(&mut self.buffer())
                {
                    let interpolated = if interpolated {
                        Some(Interpolation::new(self.curly_nest()))
                    } else {
                        None
                    };
                    self.string_literals()
                        .push(StringLiteral::Heredoc(Heredoc::new(
                            interpolated,
                            loc!(id_start, id_end),
                            token.loc.end,
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
#[test]
fn test_tSTRING_BEG_HEREDOC() {
    use crate::testing::assert_lex;
    assert_lex!(
        input = b"<<-HERE",
        token = token!(tDSTRING_BEG, loc!(0, 7)),
        setup = |lexer: &mut Lexer| {
            *lexer.curly_nest_mut() = 42;
            lexer.require_new_expr();
        },
        assert = |lexer: &Lexer| {
            assert_eq!(lexer.string_literals().size(), 1);

            assert_eq!(
                lexer.string_literals().last(),
                Some(&StringLiteral::Heredoc(Heredoc::new(
                    Some(Interpolation::new(42)),
                    loc!(3, 7),
                    7,
                    false
                )))
            );
        }
    );
}
#[test]
fn test_tCMP() {
    use crate::testing::assert_lex;
    assert_lex!(b"<=>", token!(tCMP, loc!(0, 3)));
}
#[test]
fn test_tLEQ() {
    use crate::testing::assert_lex;
    assert_lex!(b"<=", token!(tLEQ, loc!(0, 2)));
}
#[test]
fn test_tOP_ASGN_LSHIFT() {
    use crate::testing::assert_lex;
    assert_lex!(b"<<=", token!(tOP_ASGN, loc!(0, 3)));
}
#[test]
fn test_tLSHFT() {
    use crate::testing::assert_lex;
    assert_lex!(b"<<", token!(tLSHFT, loc!(0, 2)));
}
#[test]
fn test_tLT() {
    use crate::testing::assert_lex;
    assert_lex!(b"<", token!(tLT, loc!(0, 1)));
}

impl OnByte<62 /* '>' (fix highlighting) */> for Lexer {
    fn on_byte(&mut self) -> Token {
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
#[test]
fn test_tGEQ() {
    use crate::testing::assert_lex;
    assert_lex!(b">=", token!(tGEQ, loc!(0, 2)));
}
#[test]
fn test_tOP_ASGN_RSHIFT() {
    use crate::testing::assert_lex;
    assert_lex!(b">>=", token!(tOP_ASGN, loc!(0, 3)));
}
#[test]
fn test_tRSHFT() {
    use crate::testing::assert_lex;
    assert_lex!(b">>", token!(tRSHFT, loc!(0, 2)));
}
#[test]
fn test_tGT() {
    use crate::testing::assert_lex;
    assert_lex!(b">", token!(tGT, loc!(0, 1)));
}

impl OnByte<b'"'> for Lexer {
    fn on_byte(&mut self) -> Token {
        let start = self.pos();
        self.skip_byte();
        let token = token!(tDSTRING_BEG, loc!(start, start + 1));
        self.string_literals()
            .push(StringLiteral::StringInterp(StringInterp::new(
                Interpolation::new(self.curly_nest()),
                b'"',
                b'"',
            )));
        token
    }
}
#[test]
fn test_tSTRING_BEG_DQUOTE() {
    use crate::testing::assert_lex;
    assert_lex!(
        input = b"\"",
        token = token!(tDSTRING_BEG, loc!(0, 1)),
        setup = |lexer: &mut Lexer| {
            *lexer.curly_nest_mut() = 42;
        },
        assert = |lexer: &Lexer| {
            use crate::lexer::strings::types::StringInterp;
            assert_eq!(lexer.string_literals().size(), 1);

            assert_eq!(
                lexer.string_literals().last(),
                Some(&StringLiteral::StringInterp(StringInterp::new(
                    Interpolation::new(42),
                    b'"',
                    b'"'
                )))
            );
        }
    );
}

impl OnByte<b'`'> for Lexer {
    fn on_byte(&mut self) -> Token {
        // we rewrite '`' to tXSTRING_BEG on the parser level
        let start = self.pos();
        self.skip_byte();
        token!(tIDENTIFIER, loc!(start, start + 1))
    }
}
#[test]
fn test_tIDENTIFIER_backtick() {
    use crate::testing::assert_lex;
    assert_lex!(b"`", token!(tIDENTIFIER, loc!(0, 1)));
}

impl OnByte<b'\''> for Lexer {
    fn on_byte(&mut self) -> Token {
        let start = self.pos();
        self.skip_byte();
        let token = token!(tSTRING_BEG, loc!(start, start + 1));
        self.string_literals()
            .push(StringLiteral::StringPlain(StringPlain::new(b'\'', b'\'')));
        token
    }
}
#[test]
fn test_tSTRING_BEG1_SQUOTE() {
    use crate::testing::assert_lex;
    assert_lex!(
        input = b"'",
        token = token!(tSTRING_BEG, loc!(0, 1)),
        setup = |lexer: &mut Lexer| {
            *lexer.curly_nest_mut() = 42;
        },
        assert = |lexer: &Lexer| {
            assert_eq!(lexer.string_literals().size(), 1);
            assert_eq!(
                lexer.string_literals().last(),
                Some(&StringLiteral::StringPlain(StringPlain::new(b'\'', b'\'')))
            )
        }
    );
}

impl OnByte<b'?'> for Lexer {
    fn on_byte(&mut self) -> Token {
        QMark::parse(&mut self.buffer())
    }
}

impl OnByte<b'&'> for Lexer {
    fn on_byte(&mut self) -> Token {
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
#[test]
fn test_tOP_ASGN_DAMPER() {
    use crate::testing::assert_lex;
    assert_lex!(b"&&=", token!(tOP_ASGN, loc!(0, 3)));
}
#[test]
fn test_tANDOP() {
    use crate::testing::assert_lex;
    assert_lex!(b"&&", token!(tANDOP, loc!(0, 2)));
}
#[test]
fn test_tOP_ASGN_AMPER() {
    use crate::testing::assert_lex;
    assert_lex!(b"&=", token!(tOP_ASGN, loc!(0, 2)));
}
#[test]
fn test_tANDDOT() {
    use crate::testing::assert_lex;
    assert_lex!(b"&.", token!(tANDDOT, loc!(0, 2)));
}
#[test]
fn test_tAMPER() {
    use crate::testing::assert_lex;
    assert_lex!(b"&", token!(tAMPER, loc!(0, 1)));
}

impl OnByte<b'|'> for Lexer {
    fn on_byte(&mut self) -> Token {
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
#[test]
fn test_tOP_ASGN_DPIPE() {
    use crate::testing::assert_lex;
    assert_lex!(b"||=", token!(tOP_ASGN, loc!(0, 3)));
}
#[test]
fn test_tOROP() {
    use crate::testing::assert_lex;
    assert_lex!(b"||", token!(tOROP, loc!(0, 2)));
}
#[test]
fn test_tOP_ASGN_PIPE() {
    use crate::testing::assert_lex;
    assert_lex!(b"|=", token!(tOP_ASGN, loc!(0, 2)));
}
#[test]
fn test_tPIPE() {
    use crate::testing::assert_lex;
    assert_lex!(b"|", token!(tPIPE, loc!(0, 1)));
}

impl OnByte<b'+'> for Lexer {
    fn on_byte(&mut self) -> Token {
        let start = self.pos();
        self.skip_byte();
        // +@ is handled on the parser level
        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                token!(tOP_ASGN, loc!(start, start + 2))
            }
            Some(b'0'..=b'9') => {
                let mut token = parse_number(&mut self.buffer());
                token.loc.start = start;
                token
            }
            _ => token!(tPLUS, loc!(start, start + 1)),
        }
    }
}
#[test]
fn test_tOP_ASGN_PLUS() {
    use crate::testing::assert_lex;
    assert_lex!(b"+=", token!(tOP_ASGN, loc!(0, 2)));
}
#[test]
fn test_tPLUS_NUMBER() {
    use crate::testing::assert_lex;
    assert_lex!(b"+1", token!(tINTEGER, loc!(0, 2)));
}
#[test]
fn test_tPLUS() {
    use crate::testing::assert_lex;
    assert_lex!(b"+", token!(tPLUS, loc!(0, 1)));
}

impl OnByte<b'-'> for Lexer {
    fn on_byte(&mut self) -> Token {
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
#[test]
fn test_tOP_ASGN_MINUS() {
    use crate::testing::assert_lex;
    assert_lex!(b"-=", token!(tOP_ASGN, loc!(0, 2)));
}
#[test]
fn test_tLAMBDA() {
    use crate::testing::assert_lex;
    assert_lex!(b"->", token!(tLAMBDA, loc!(0, 2)));
}
#[test]
fn test_tMINUS() {
    use crate::testing::assert_lex;
    assert_lex!(b"-", token!(tMINUS, loc!(0, 1)));
}
#[test]
fn test_tUMINUS() {
    use crate::testing::assert_lex;
    assert_lex!(b"-5", token!(tUMINUS, loc!(0, 1)));
}

impl OnByte<b'.'> for Lexer {
    fn on_byte(&mut self) -> Token {
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
                while matches!(self.buffer().byte_at(end), Some(b'0'..=b'9')) {
                    end += 1;
                }
                panic!(
                    "no .<digit> floating literal anymore; put 0 before dot ({:?})",
                    self.buffer().slice(start, end)
                );
            }
            _ => token!(tDOT, loc!(start, start + 1)),
        }
    }
}
#[test]
fn test_tDOT3() {
    use crate::testing::assert_lex;
    assert_lex!(b"...", token!(tDOT3, loc!(0, 3)));
}
#[test]
fn test_tDOT2() {
    use crate::testing::assert_lex;
    assert_lex!(b"..", token!(tDOT2, loc!(0, 2)));
}
#[test]
fn test_tDOT() {
    use crate::testing::assert_lex;
    assert_lex!(b".", token!(tDOT, loc!(0, 1)));
}

impl OnByte<b')'> for Lexer {
    fn on_byte(&mut self) -> Token {
        let start = self.pos();
        self.skip_byte();
        if self.paren_nest() > 0 {
            *self.paren_nest_mut() -= 1;
        } else {
            panic!("negative paren_nest");
        }

        token!(tRPAREN, loc!(start, start + 1))
    }
}
#[test]
fn test_tRPAREN() {
    use crate::testing::assert_lex;
    assert_lex!(
        input = b")",
        token = token!(tRPAREN, loc!(0, 1)),
        setup = |lexer: &mut Lexer| {
            *lexer.paren_nest_mut() = 1;
        },
        assert = |_lexer: &Lexer| {}
    );
}

impl OnByte<b']'> for Lexer {
    fn on_byte(&mut self) -> Token {
        let start = self.pos();
        self.skip_byte();
        if self.brack_nest() > 0 {
            *self.brack_nest_mut() -= 1;
        } else {
            panic!("negative brack_nest");
        }
        token!(tRBRACK, loc!(start, start + 1))
    }
}
#[test]
fn test_tRBRACK() {
    use crate::testing::assert_lex;
    assert_lex!(
        input = b"]",
        token = token!(tRBRACK, loc!(0, 1)),
        setup = |lexer: &mut Lexer| {
            *lexer.brack_nest_mut() = 1;
        },
        assert = |_lexer: &Lexer| {}
    );
}

impl OnByte<b'}'> for Lexer {
    fn on_byte(&mut self) -> Token {
        let start = self.pos();
        self.skip_byte();
        if self.curly_nest() > 0 {
            *self.curly_nest_mut() -= 1;
        } else {
            panic!("negative curly_nest");
        }
        token!(tRCURLY, loc!(start, start + 1))
    }
}
#[test]
fn test_tRCURLY() {
    use crate::testing::assert_lex;
    assert_lex!(
        input = b"}",
        token = token!(tRCURLY, loc!(0, 1)),
        setup = |lexer: &mut Lexer| {
            *lexer.curly_nest_mut() = 1;
        },
        assert = |_lexer: &Lexer| {}
    );
}

impl OnByte<b':'> for Lexer {
    fn on_byte(&mut self) -> Token {
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
                self.string_literals()
                    .push(StringLiteral::Symbol(Symbol::new(true, self.curly_nest())));
                return token;
            }
            Some(b'\'') => {
                // :'...' symbol
                self.skip_byte();
                let token = token!(tSYMBEG, loc!(start, start + 2));
                self.string_literals()
                    .push(StringLiteral::Symbol(Symbol::new(false, self.curly_nest())));
                return token;
            }
            _ => {}
        }

        // Plain symbols are handled on the parser level
        // by concatenating tCOLON and tIDENTIFIER/tCONSTANT/tIVAR/tCVAR/tGVAR

        token!(tCOLON, loc!(start, start + 1))
    }
}
#[test]
fn test_tCOLON2() {
    use crate::testing::assert_lex;
    assert_lex!(b"::", token!(tCOLON2, loc!(0, 2)));
}
#[test]
fn test_tDSYMBEG() {
    use crate::testing::assert_lex;
    assert_lex!(
        input = b":\"",
        token = token!(tDSYMBEG, loc!(0, 2)),
        setup = |lexer: &mut Lexer| {
            *lexer.curly_nest_mut() = 42;
        },
        assert = |lexer: &Lexer| {
            use crate::lexer::strings::types::Symbol;

            assert_eq!(lexer.string_literals().size(), 1);
            assert_eq!(
                lexer.string_literals().last(),
                Some(&StringLiteral::Symbol(Symbol::new(true, 42)))
            )
        }
    );
}
#[test]
fn test_tSYMBEG() {
    use crate::testing::assert_lex;
    assert_lex!(
        input = b":'",
        token = token!(tSYMBEG, loc!(0, 2)),
        setup = |lexer: &mut Lexer| {
            *lexer.curly_nest_mut() = 42;
        },
        assert = |lexer: &Lexer| {
            use crate::lexer::strings::types::Symbol;

            assert_eq!(lexer.string_literals().size(), 1);
            assert_eq!(
                lexer.string_literals().last(),
                Some(&StringLiteral::Symbol(Symbol::new(false, 42)))
            )
        }
    );
}
#[test]
fn test_tCOLON() {
    use crate::testing::assert_lex;
    assert_lex!(b":", token!(tCOLON, loc!(0, 1)));
}

impl OnByte<b'/'> for Lexer {
    fn on_byte(&mut self) -> Token {
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
#[test]
fn test_tOP_ASGN_DIV() {
    use crate::testing::assert_lex;
    assert_lex!(b"/=", token!(tOP_ASGN, loc!(0, 2)));
}
#[test]
fn test_tDIVIDE() {
    use crate::testing::assert_lex;
    assert_lex!(b"/", token!(tDIVIDE, loc!(0, 1)));
}

impl OnByte<b'^'> for Lexer {
    fn on_byte(&mut self) -> Token {
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
#[test]
fn test_tOP_ASGN_CARET() {
    use crate::testing::assert_lex;
    assert_lex!(b"^=", token!(tOP_ASGN, loc!(0, 2)));
}
#[test]
fn test_tCARET() {
    use crate::testing::assert_lex;
    assert_lex!(b"^", token!(tCARET, loc!(0, 1)));
}

impl OnByte<b';'> for Lexer {
    fn on_byte(&mut self) -> Token {
        let start = self.pos();
        self.skip_byte();
        token!(tSEMI, loc!(start, start + 1))
    }
}
#[test]
fn test_tSEMI() {
    use crate::testing::assert_lex;
    assert_lex!(b";", token!(tSEMI, loc!(0, 1)));
}

impl OnByte<b','> for Lexer {
    fn on_byte(&mut self) -> Token {
        let start = self.pos();
        self.skip_byte();
        token!(tCOMMA, loc!(start, start + 1))
    }
}
#[test]
fn test_tCOMMA() {
    use crate::testing::assert_lex;
    assert_lex!(b",", token!(tCOMMA, loc!(0, 1)));
}

impl OnByte<b'~'> for Lexer {
    fn on_byte(&mut self) -> Token {
        let start = self.pos();
        self.skip_byte();
        // ~@ is handled on the parser level
        token!(tTILDE, loc!(start, start + 1))
    }
}
#[test]
fn test_tTILDE() {
    use crate::testing::assert_lex;
    assert_lex!(b"~", token!(tTILDE, loc!(0, 1)));
}

impl OnByte<b'('> for Lexer {
    fn on_byte(&mut self) -> Token {
        let start = self.pos();
        self.skip_byte();
        *self.paren_nest_mut() += 1;
        token!(tLPAREN, loc!(start, start + 1))
    }
}
#[test]
fn test_tLPAREN() {
    use crate::testing::assert_lex;
    assert_lex!(b"(", token!(tLPAREN, loc!(0, 1)));
}

impl OnByte<b'['> for Lexer {
    fn on_byte(&mut self) -> Token {
        let start = self.pos();
        self.skip_byte();
        *self.brack_nest_mut() += 1;
        token!(tLBRACK, loc!(start, start + 1))
    }
}
#[test]
fn test_tLBRACK() {
    use crate::testing::assert_lex;
    assert_lex!(b"[", token!(tLBRACK, loc!(0, 1)));
}

impl OnByte<b'{'> for Lexer {
    fn on_byte(&mut self) -> Token {
        let start = self.pos();
        self.skip_byte();
        *self.curly_nest_mut() += 1;
        token!(tLCURLY, loc!(start, start + 1))
    }
}

impl OnByte<b'\\'> for Lexer {
    fn on_byte(&mut self) -> Token {
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
#[test]
fn test_tESCAPED_NL() {
    use crate::testing::assert_lex;
    assert_lex!(b"\\\nTEST_TOKEN", token!(tTEST_TOKEN, loc!(2, 12)));
}
#[test]
fn test_tESCAPED_SP() {
    use crate::testing::assert_lex;
    assert_lex!(b"\\ ", token!(tSP, loc!(0, 2)));
}
#[test]
fn test_tESCAPED_TAB() {
    use crate::testing::assert_lex;
    assert_lex!(b"\\\t", token!(tSLASH_T, loc!(0, 2)));
}
#[test]
fn test_tESCAPED_LF() {
    use crate::testing::assert_lex;
    assert_lex!(b"\\\x0c", token!(tSLASH_F, loc!(0, 2)));
}
#[test]
fn test_tESCAPED_CR() {
    use crate::testing::assert_lex;
    assert_lex!(b"\\\r", token!(tSLASH_R, loc!(0, 2)));
}
#[test]
fn test_tESCAPED_VTAB() {
    use crate::testing::assert_lex;
    assert_lex!(b"\\\x0b", token!(tVTAB, loc!(0, 2)));
}

impl OnByte<b'_'> for Lexer {
    fn on_byte(&mut self) -> Token {
        let start = self.pos();

        let prev_byte = start
            .checked_sub(1)
            .map(|idx| self.buffer().byte_at(idx))
            .flatten();
        match prev_byte {
            // prev byte is either
            //   + None (i.e. it's the first byte of the file)
            //   + Some(b'\n')
            // AND it's "__END__" sequence
            None | Some(b'\n') if self.buffer().lookahead(b"__END__") => {
                return token!(tEOF, loc!(start, start));
            }
            _ => {}
        }

        // otherwise it's a `_foo`/`_foo?`/`_foo!` identifier
        Ident::parse(&mut self.buffer())
    }
}
#[test]
fn test_tEOF_at__END__() {
    use crate::testing::assert_lex;
    assert_lex!(b"__END__", token!(tEOF, loc!(0, 0)));
}
#[test]
fn test_tEOF_at_NL___END__() {
    use crate::testing::assert_lex;
    assert_lex!(b"\n__END__", token!(tEOF, loc!(1, 1)));
}
#[test]
fn test_underscore_ident() {
    use crate::testing::assert_lex;
    assert_lex!(b"_foo", token!(tIDENTIFIER, loc!(0, 4)));
}
#[test]
fn test_underscore_fid() {
    use crate::testing::assert_lex;
    assert_lex!(b"_foo?", token!(tFID, loc!(0, 5)));
}
