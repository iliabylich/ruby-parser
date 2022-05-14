use crate::lexer::{assert_lex, Lexer, OnByte, StringLiteral};
use crate::token::{Loc, Token, TokenValue};

impl OnByte<b'#'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        todo!("parse_comment");
        #[allow(unreachable_code)]
        Err(())
    }
}
// assert_lex!(test_tCOMMENT_INLINE, "# foo", tCOMMENT(b"# foo"), 0..6);

impl OnByte<b'*'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;
        let token = if let Some(b'*') = self.current_byte() {
            self.skip_byte();
            if let Some(b'=') = self.current_byte() {
                self.skip_byte();
                Token(TokenValue::tOP_ASGN(b"**="), Loc(start, self.pos()))
            } else {
                Token(TokenValue::tPOW, Loc(start, self.pos()))
            }
        } else if let Some(b'=') = self.current_byte() {
            self.skip_byte();
            Token(TokenValue::tOP_ASGN(b"*="), Loc(start, self.pos()))
        } else {
            Token(TokenValue::tSTAR, Loc(start, self.pos()))
        };
        self.add_token(token);
        Ok(())
    }
}
assert_lex!(test_tSTAR, "*", tSTAR, 0..1);
assert_lex!(test_tOP_ASGN_STAR, "*=", tOP_ASGN(b"*="), 0..2);
assert_lex!(test_tPOW, "**", tPOW, 0..2);
assert_lex!(test_tOP_ASGN_DSTAR, "**=", tOP_ASGN(b"**="), 0..3);

impl OnByte<b'!'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;

        // !@ is handled on the parser level
        let token = if let Some(b'=') = self.current_byte() {
            self.skip_byte();
            Token(TokenValue::tNEQ, Loc(start, self.pos()))
        } else if let Some(b'~') = self.current_byte() {
            self.skip_byte();
            Token(TokenValue::tNMATCH, Loc(start, self.pos()))
        } else {
            Token(TokenValue::tBANG, Loc(start, self.pos()))
        };
        self.add_token(token);
        Ok(())
    }
}
assert_lex!(test_tNEQ, "!=", tNEQ, 0..2);
assert_lex!(test_tNMATCH, "!~", tNMATCH, 0..2);
assert_lex!(test_tBANG, "!", tBANG, 0..1);

impl OnByte<b'='> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;

        let token = if self.buffer.lookahead(b"begin") {
            Token(TokenValue::tEMBEDDED_COMMENT_START, Loc(start, self.pos()))
        } else if let Some(b'=') = self.current_byte() {
            self.skip_byte();
            if let Some(b'=') = self.current_byte() {
                self.skip_byte();
                Token(TokenValue::tEQQ, Loc(start, self.pos()))
            } else {
                Token(TokenValue::tEQ, Loc(start, self.pos()))
            }
        } else if let Some(b'~') = self.current_byte() {
            self.skip_byte();
            Token(TokenValue::tMATCH, Loc(start, self.pos()))
        } else if let Some(b'>') = self.current_byte() {
            self.skip_byte();
            Token(TokenValue::tASSOC, Loc(start, self.pos()))
        } else {
            Token(TokenValue::tEQL, Loc(start, self.pos()))
        };
        self.add_token(token);
        Ok(())
    }
}
assert_lex!(
    test_tEMBEDDED_COMMENT_START,
    "=begin",
    tEMBEDDED_COMMENT_START,
    0..1
);
assert_lex!(test_tEQQ, "===", tEQQ, 0..3);
assert_lex!(test_tEQ, "==", tEQ, 0..2);
assert_lex!(test_tMATCH, "=~", tMATCH, 0..2);
assert_lex!(test_tASSOC, "=>", tASSOC, 0..2);
assert_lex!(test_tEQL, "=", tEQL, 0..1);

impl OnByte<b'<'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;
        // Check if heredoc id
        if let Some(b'<') = self.current_byte() {
            if let Some(prev_idx) = start.checked_sub(1) {
                if self.buffer.byte_at(prev_idx) == Some(b' ') {
                    if let Some(_here_id) = self.tokenize_heredoc_id() {
                        return Ok(());
                    }
                }
            }
        }
        // Otherwise just an operator
        if let Some(b'=') = self.current_byte() {
            self.skip_byte();
            if let Some(b'>') = self.current_byte() {
                self.skip_byte();
                self.add_token(Token(TokenValue::tCMP, Loc(start, self.pos())));
            } else {
                self.add_token(Token(TokenValue::tLEQ, Loc(start, self.pos())));
            }
        } else if let Some(b'<') = self.current_byte() {
            self.skip_byte();
            if let Some(b'=') = self.current_byte() {
                self.skip_byte();
                self.add_token(Token(TokenValue::tOP_ASGN(b"<<="), Loc(start, self.pos())));
            } else {
                self.add_token(Token(TokenValue::tLSHFT, Loc(start, self.pos())));
            }
        } else {
            self.add_token(Token(TokenValue::tLT, Loc(start, self.pos())));
        }
        Ok(())
    }
}
// assert_lex!(test_tSTRING_BEG_HEREDOC, "<<-HERE", 0..5);
assert_lex!(test_tCMP, "<=>", tCMP, 0..3);
assert_lex!(test_tLEQ, "<=", tLEQ, 0..2);
assert_lex!(test_tOP_ASGN_LSHIFT, "<<=", tOP_ASGN(b"<<="), 0..3);
assert_lex!(test_tLSHFT, "<<", tLSHFT, 0..2);
assert_lex!(test_tLT, "<", tLT, 0..1);

impl OnByte<b'>'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;
        if let Some(b'=') = self.current_byte() {
            self.skip_byte();
            self.add_token(Token(TokenValue::tGEQ, Loc(start, self.pos())));
        } else if let Some(b'>') = self.current_byte() {
            self.skip_byte();
            if let Some(b'=') = self.current_byte() {
                self.skip_byte();
                self.add_token(Token(TokenValue::tOP_ASGN(b">>="), Loc(start, self.pos())));
            } else {
                self.add_token(Token(TokenValue::tRSHFT, Loc(start, self.pos())));
            }
        } else {
            self.add_token(Token(TokenValue::tGT, Loc(start, self.pos())));
        }

        Ok(())
    }
}
assert_lex!(test_tGEQ, ">=", tGEQ, 0..2);
assert_lex!(test_tOP_ASGN_RSHIFT, ">>=", tOP_ASGN(b">>="), 0..3);
assert_lex!(test_tRSHFT, ">>", tRSHFT, 0..2);
assert_lex!(test_tGT, ">", tGT, 0..1);

impl OnByte<b'"'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;
        self.add_token(Token(
            TokenValue::tSTRING_BEG(b"\""),
            Loc(start, self.pos()),
        ));
        self.string_literals.push(StringLiteral::Plain {
            supports_interpolation: true,
            currently_in_interpolation: false,
            ends_with: b"\"",
            interpolation_started_with_curly_level: self.curly_braces,
        });
        Ok(())
    }
}
// assert_lex!(test_tSTRING_BEG_DQUOTE, "\"", tSTRING_BEG(b"\""), 0..1);

impl OnByte<b'`'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        todo!("unclear what to do here?? MRI does state-dependent analysis")
    }
}

impl OnByte<b'\''> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;
        self.add_token(Token(TokenValue::tSTRING_BEG(b"'"), Loc(start, self.pos())));
        self.string_literals.push(StringLiteral::Plain {
            supports_interpolation: false,
            currently_in_interpolation: false,
            ends_with: b"'",
            interpolation_started_with_curly_level: 0,
        });
        Ok(())
    }
}
// assert_lex!(test_tSTRING_BEG1_SQUOTE, "'", tSTRING_BEG(b"'"), 0..1);

impl OnByte<b'?'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        todo!("parse_qmark")
    }
}

impl OnByte<b'&'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;
        if let Some(b'&') = self.current_byte() {
            self.skip_byte();
            if let Some(b'=') = self.current_byte() {
                self.skip_byte();
                self.add_token(Token(TokenValue::tOP_ASGN(b"&&="), Loc(start, self.pos())));
            } else {
                self.add_token(Token(TokenValue::tANDOP, Loc(start, self.pos())));
            }
        } else if let Some(b'=') = self.current_byte() {
            self.skip_byte();
            self.add_token(Token(TokenValue::tOP_ASGN(b"&="), Loc(start, self.pos())));
        } else if let Some(b'.') = self.current_byte() {
            self.skip_byte();
            self.add_token(Token(TokenValue::tANDDOT, Loc(start, self.pos())));
        } else {
            self.add_token(Token(TokenValue::tAMPER, Loc(start, self.pos())));
        }
        Ok(())
    }
}
assert_lex!(test_tOP_ASGN_DAMPER, "&&=", tOP_ASGN(b"&&="), 0..3);
assert_lex!(test_tANDOP, "&&", tANDOP, 0..2);
assert_lex!(test_tOP_ASGN_AMPER, "&=", tOP_ASGN(b"&="), 0..2);
assert_lex!(test_tANDDOT, "&.", tANDDOT, 0..2);
assert_lex!(test_tAMPER, "&", tAMPER, 0..1);

impl OnByte<b'|'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;
        if let Some(b'|') = self.current_byte() {
            self.skip_byte();
            if let Some(b'=') = self.current_byte() {
                self.skip_byte();
                self.add_token(Token(TokenValue::tOP_ASGN(b"||="), Loc(start, self.pos())));
            } else {
                self.add_token(Token(TokenValue::tOROP, Loc(start, self.pos())));
            }
        } else if let Some(b'=') = self.current_byte() {
            self.skip_byte();
            self.add_token(Token(TokenValue::tOP_ASGN(b"|="), Loc(start, self.pos())));
        } else {
            self.add_token(Token(TokenValue::tPIPE, Loc(start, self.pos())));
        }
        Ok(())
    }
}
assert_lex!(test_tOP_ASGN_DPIPE, "||=", tOP_ASGN(b"||="), 0..3);
assert_lex!(test_tOROP, "||", tOROP, 0..2);
assert_lex!(test_tOP_ASGN_PIPE, "|=", tOP_ASGN(b"|="), 0..2);
assert_lex!(test_tPIPE, "|", tPIPE, 0..1);

impl OnByte<b'+'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        // TODO: extend
        let start = self.pos() - 1;
        self.add_token(Token(TokenValue::tPLUS, Loc(start, self.pos())));
        Ok(())
    }
}
assert_lex!(test_tPLUS, "+", tPLUS, 0..1);

impl OnByte<b'-'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        // TODO: extend
        let start = self.pos() - 1;
        self.add_token(Token(TokenValue::tMINUS, Loc(start, self.pos())));
        Ok(())
    }
}
assert_lex!(test_tMINUS, "-", tMINUS, 0..1);

impl OnByte<b'.'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        todo!()
    }
}

impl OnByte<b')'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;

        self.add_token(Token(TokenValue::tRPAREN, Loc(start, self.pos())));
        Ok(())
    }
}
assert_lex!(test_tRPAREN, ")", tRPAREN, 0..1);

impl OnByte<b']'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;
        self.add_token(Token(TokenValue::tRBRACK, Loc(start, self.pos())));
        Ok(())
    }
}
assert_lex!(test_tRBRACK, "]", tRBRACK, 0..1);

impl OnByte<b'}'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;
        self.add_token(Token(TokenValue::tRCURLY, Loc(start, self.pos())));
        Ok(())
    }
}
assert_lex!(test_tRCURLY, "}", tRCURLY, 0..1);

impl OnByte<b':'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        todo!()
    }
}

impl OnByte<b'/'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;
        self.add_token(Token(TokenValue::tDIVIDE, Loc(start, self.pos())));
        Ok(())
    }
}
assert_lex!(test_tDIVIDE, "/", tDIVIDE, 0..1);

impl OnByte<b'^'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        todo!()
    }
}

impl OnByte<b';'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        todo!()
    }
}

impl OnByte<b','> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        todo!()
    }
}

impl OnByte<b'~'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        todo!()
    }
}

impl OnByte<b'('> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;
        self.add_token(Token(TokenValue::tLPAREN, Loc(start, self.pos())));
        Ok(())
    }
}
assert_lex!(test_tLPAREN, "(", tLPAREN, 0..1);

impl OnByte<b'['> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;
        self.add_token(Token(TokenValue::tLBRACK, Loc(start, self.pos())));
        Ok(())
    }
}
assert_lex!(test_tLBRACK, "[", tLBRACK, 0..1);

impl OnByte<b'{'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;
        self.add_token(Token(TokenValue::tLCURLY, Loc(start, self.pos())));
        Ok(())
    }
}

impl OnByte<b'\\'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        todo!()
    }
}

impl OnByte<b'%'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        todo!()
    }
}

impl OnByte<b'$'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        todo!()
    }
}

impl OnByte<b'@'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        todo!()
    }
}

impl OnByte<b'_'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        todo!()
    }
}
