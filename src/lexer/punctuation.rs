use crate::lexer::{assert_lex, Lexer, OnByte, StringLiteral, StringLiteralMetadata};
use crate::token::{Loc, Token, TokenValue};

use crate::lexer::ident::parse_ident;
use crate::lexer::numbers::parse_number;

impl<'a> OnByte<'a, b'#'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let _start = self.pos();

        todo!("parse_comment");
    }
}
// assert_lex!(test_tCOMMENT_INLINE, "# foo", tCOMMENT(b"# foo"), 0..6);

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
                        Token(TokenValue::tOP_ASGN(b"**="), Loc(start, self.pos()))
                    }
                    _ => Token(TokenValue::tPOW, Loc(start, self.pos())),
                }
            }
            Some(b'=') => {
                self.skip_byte();
                Token(TokenValue::tOP_ASGN(b"*="), Loc(start, self.pos()))
            }
            _ => Token(TokenValue::tSTAR, Loc(start, self.pos())),
        }
    }
}
assert_lex!(test_tSTAR, "*", tSTAR, 0..1);
assert_lex!(test_tOP_ASGN_STAR, "*=", tOP_ASGN(b"*="), 0..2);
assert_lex!(test_tPOW, "**", tPOW, 0..2);
assert_lex!(test_tOP_ASGN_DSTAR, "**=", tOP_ASGN(b"**="), 0..3);

impl<'a> OnByte<'a, b'!'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();

        // !@ is handled on the parser level
        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                Token(TokenValue::tNEQ, Loc(start, self.pos()))
            }
            Some(b'~') => {
                self.skip_byte();
                Token(TokenValue::tNMATCH, Loc(start, self.pos()))
            }
            _ => Token(TokenValue::tBANG, Loc(start, self.pos())),
        }
    }
}
assert_lex!(test_tNEQ, "!=", tNEQ, 0..2);
assert_lex!(test_tNMATCH, "!~", tNMATCH, 0..2);
assert_lex!(test_tBANG, "!", tBANG, 0..1);

impl<'a> OnByte<'a, b'='> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();

        if self.buffer.const_lookahead(b"begin") {
            return Token(TokenValue::tEMBEDDED_COMMENT_START, Loc(start, self.pos()));
        }

        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'=') => {
                        self.skip_byte();
                        Token(TokenValue::tEQQ, Loc(start, self.pos()))
                    }
                    _ => Token(TokenValue::tEQ, Loc(start, self.pos())),
                }
            }
            Some(b'~') => {
                self.skip_byte();
                Token(TokenValue::tMATCH, Loc(start, self.pos()))
            }
            Some(b'>') => {
                self.skip_byte();
                Token(TokenValue::tASSOC, Loc(start, self.pos()))
            }
            _ => Token(TokenValue::tEQL, Loc(start, self.pos())),
        }
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

impl<'a> OnByte<'a, b'<'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        // Check if heredoc id
        if let Some(b'<') = self.current_byte() {
            if let Some(prev_idx) = start.checked_sub(1) {
                if self.buffer.byte_at(prev_idx) == Some(b' ') {
                    if let Some(_here_id) = self.tokenize_heredoc_id() {
                        todo!("heredoc_id");
                    }
                }
            }
        }

        // Otherwise just an operator
        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'>') => {
                        self.skip_byte();
                        Token(TokenValue::tCMP, Loc(start, self.pos()))
                    }
                    _ => Token(TokenValue::tLEQ, Loc(start, self.pos())),
                }
            }
            Some(b'<') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'=') => {
                        self.skip_byte();
                        Token(TokenValue::tOP_ASGN(b"<<="), Loc(start, self.pos()))
                    }
                    _ => Token(TokenValue::tLSHFT, Loc(start, self.pos())),
                }
            }
            _ => Token(TokenValue::tLT, Loc(start, self.pos())),
        }
    }
}
// assert_lex!(test_tSTRING_BEG_HEREDOC, "<<-HERE", 0..5);
assert_lex!(test_tCMP, "<=>", tCMP, 0..3);
assert_lex!(test_tLEQ, "<=", tLEQ, 0..2);
assert_lex!(test_tOP_ASGN_LSHIFT, "<<=", tOP_ASGN(b"<<="), 0..3);
assert_lex!(test_tLSHFT, "<<", tLSHFT, 0..2);
assert_lex!(test_tLT, "<", tLT, 0..1);

impl<'a> OnByte<'a, b'>'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                Token(TokenValue::tGEQ, Loc(start, self.pos()))
            }
            Some(b'>') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'=') => {
                        self.skip_byte();
                        Token(TokenValue::tOP_ASGN(b">>="), Loc(start, self.pos()))
                    }
                    _ => Token(TokenValue::tRSHFT, Loc(start, self.pos())),
                }
            }
            _ => Token(TokenValue::tGT, Loc(start, self.pos())),
        }
    }
}
assert_lex!(test_tGEQ, ">=", tGEQ, 0..2);
assert_lex!(test_tOP_ASGN_RSHIFT, ">>=", tOP_ASGN(b">>="), 0..3);
assert_lex!(test_tRSHFT, ">>", tRSHFT, 0..2);
assert_lex!(test_tGT, ">", tGT, 0..1);

impl<'a> OnByte<'a, b'"'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        let token = Token(TokenValue::tSTRING_BEG(b"\""), Loc(start, self.pos()));
        self.string_literals.push(StringLiteral {
            supports_interpolation: true,
            currently_in_interpolation: false,
            ends_with: b"\"",
            interpolation_started_with_curly_level: self.curly_nest,
            metadata: StringLiteralMetadata::Plain,
        });
        token
    }
}
assert_lex!(
    test_tSTRING_BEG_DQUOTE,
    "\"",
    tSTRING_BEG(b"\""),
    0..1,
    setup = |lexer: &mut Lexer| {
        lexer.curly_nest = 42;
    },
    assert = |lexer: &Lexer| {
        assert_eq!(lexer.string_literals.size(), 1);

        assert_eq!(
            lexer.string_literals.last(),
            Some(StringLiteral {
                supports_interpolation: true,
                currently_in_interpolation: false,
                ends_with: b"\"",
                interpolation_started_with_curly_level: 42,
                metadata: StringLiteralMetadata::Plain
            })
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
        let token = Token(TokenValue::tSTRING_BEG(b"'"), Loc(start, self.pos()));
        self.string_literals.push(StringLiteral {
            supports_interpolation: false,
            currently_in_interpolation: false,
            ends_with: b"'",
            interpolation_started_with_curly_level: 0,
            metadata: StringLiteralMetadata::Plain,
        });
        token
    }
}
assert_lex!(
    test_tSTRING_BEG1_SQUOTE,
    "'",
    tSTRING_BEG(b"'"),
    0..1,
    setup = |lexer: &mut Lexer| {
        lexer.curly_nest = 42;
    },
    assert = |lexer: &Lexer| {
        assert_eq!(lexer.string_literals.size(), 1);
        assert_eq!(
            lexer.string_literals.last(),
            Some(StringLiteral {
                supports_interpolation: false,
                currently_in_interpolation: false,
                ends_with: b"'",
                interpolation_started_with_curly_level: 0,
                metadata: StringLiteralMetadata::Plain
            })
        )
    }
);

impl<'a> OnByte<'a, b'?'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        todo!("parse_qmark")
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
                        Token(TokenValue::tOP_ASGN(b"&&="), Loc(start, self.pos()))
                    }
                    _ => Token(TokenValue::tANDOP, Loc(start, self.pos())),
                }
            }
            Some(b'=') => {
                self.skip_byte();
                Token(TokenValue::tOP_ASGN(b"&="), Loc(start, self.pos()))
            }
            Some(b'.') => {
                self.skip_byte();
                Token(TokenValue::tANDDOT, Loc(start, self.pos()))
            }
            _ => Token(TokenValue::tAMPER, Loc(start, self.pos())),
        }
    }
}
assert_lex!(test_tOP_ASGN_DAMPER, "&&=", tOP_ASGN(b"&&="), 0..3);
assert_lex!(test_tANDOP, "&&", tANDOP, 0..2);
assert_lex!(test_tOP_ASGN_AMPER, "&=", tOP_ASGN(b"&="), 0..2);
assert_lex!(test_tANDDOT, "&.", tANDDOT, 0..2);
assert_lex!(test_tAMPER, "&", tAMPER, 0..1);

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
                        Token(TokenValue::tOP_ASGN(b"||="), Loc(start, self.pos()))
                    }
                    _ => Token(TokenValue::tOROP, Loc(start, self.pos())),
                }
            }
            Some(b'=') => {
                self.skip_byte();
                Token(TokenValue::tOP_ASGN(b"|="), Loc(start, self.pos()))
            }
            _ => Token(TokenValue::tPIPE, Loc(start, self.pos())),
        }
    }
}
assert_lex!(test_tOP_ASGN_DPIPE, "||=", tOP_ASGN(b"||="), 0..3);
assert_lex!(test_tOROP, "||", tOROP, 0..2);
assert_lex!(test_tOP_ASGN_PIPE, "|=", tOP_ASGN(b"|="), 0..2);
assert_lex!(test_tPIPE, "|", tPIPE, 0..1);

impl<'a> OnByte<'a, b'+'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        // +@ is handled on the parser level
        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                Token(TokenValue::tOP_ASGN(b"+="), Loc(start, self.pos()))
            }
            Some(b'0'..=b'9') => {
                let mut token = parse_number(&mut self.buffer);
                token.1 .0 = start;
                let new_value = self.slice(token.loc().begin(), token.loc().end());
                match &mut token.0 {
                    TokenValue::tINTEGER(v) => *v = new_value,
                    other => unreachable!("Unsupported token value {:?}", other),
                };
                token
            }
            _ => Token(TokenValue::tPLUS, Loc(start, self.pos())),
        }
    }
}
assert_lex!(test_tOP_ASGN_PLUS, "+=", tOP_ASGN(b"+="), 0..2);
assert_lex!(test_tPLUS_NUMBER, "+1", tINTEGER(b"+1"), 0..2);
assert_lex!(test_tPLUS, "+", tPLUS, 0..1);

impl<'a> OnByte<'a, b'-'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        // -@ is handled on the parser level
        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                Token(TokenValue::tOP_ASGN(b"-="), Loc(start, self.pos()))
            }
            Some(b'>') => {
                self.skip_byte();
                Token(TokenValue::tLAMBDA, Loc(start, self.pos()))
            }
            Some(b'0'..=b'9') => Token(TokenValue::tUMINUS, Loc(start, self.pos())),
            _ => Token(TokenValue::tMINUS, Loc(start, self.pos())),
        }
    }
}
assert_lex!(test_tOP_ASGN_MINUS, "-=", tOP_ASGN(b"-="), 0..2);
assert_lex!(test_tLAMBDA, "->", tLAMBDA, 0..2);
assert_lex!(test_tMINUS, "-", tMINUS, 0..1);
assert_lex!(test_tUMINUS, "-5", tUMINUS, 0..1);

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
                        Token(TokenValue::tDOT3, Loc(start, self.pos()))
                    }
                    _ => Token(TokenValue::tDOT2, Loc(start, self.pos())),
                }
            }
            Some(b'0'..=b'9') => {
                todo!("Handle .<n> case as error?? Skip all number until NaN found");
            }
            _ => Token(TokenValue::tDOT, Loc(start, self.pos())),
        }
    }
}
assert_lex!(test_tDOT3, "...", tDOT3, 0..3);
assert_lex!(test_tDOT2, "..", tDOT2, 0..2);
assert_lex!(test_tDOT, ".", tDOT, 0..1);

impl<'a> OnByte<'a, b')'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        if self.paren_nest > 0 {
            self.paren_nest -= 1;
        } else {
            todo!("Report paren_nest error");
        }

        Token(TokenValue::tRPAREN, Loc(start, self.pos()))
    }
}
assert_lex!(
    test_tRPAREN,
    ")",
    tRPAREN,
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
            todo!("Report brack_nest error");
        }
        Token(TokenValue::tRBRACK, Loc(start, self.pos()))
    }
}
assert_lex!(
    test_tRBRACK,
    "]",
    tRBRACK,
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
            todo!("Report curly_nest error");
        }
        Token(TokenValue::tRCURLY, Loc(start, self.pos()))
    }
}
assert_lex!(
    test_tRCURLY,
    "}",
    tRCURLY,
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
                Token(TokenValue::tCOLON2, Loc(start, self.pos()))
            }
            Some(b'"') => {
                // :"..." symbol
                self.skip_byte();
                let token = Token(TokenValue::tDSYMBEG, Loc(start, self.pos()));
                self.string_literals.push(StringLiteral {
                    supports_interpolation: true,
                    currently_in_interpolation: false,
                    ends_with: b" ",
                    interpolation_started_with_curly_level: self.curly_nest,
                    metadata: StringLiteralMetadata::Plain,
                });
                token
            }
            Some(b'\'') => {
                // :'...' symbol
                self.skip_byte();
                let token = Token(TokenValue::tSYMBEG, Loc(start, self.pos()));
                self.string_literals.push(StringLiteral {
                    supports_interpolation: false,
                    currently_in_interpolation: false,
                    ends_with: b" ",
                    interpolation_started_with_curly_level: 0,
                    metadata: StringLiteralMetadata::Plain,
                });
                token
            }
            _ => Token(TokenValue::tCOLON, Loc(start, self.pos())),
        }
    }
}
assert_lex!(test_tCOLON2, "::", tCOLON2, 0..2);
assert_lex!(
    test_tDSYMBEG,
    ":\"",
    tDSYMBEG,
    0..2,
    setup = |lexer: &mut Lexer| {
        lexer.curly_nest = 42;
    },
    assert = |lexer: &Lexer| {
        assert_eq!(lexer.string_literals.size(), 1);
        assert_eq!(
            lexer.string_literals.last(),
            Some(StringLiteral {
                supports_interpolation: true,
                currently_in_interpolation: false,
                ends_with: b" ",
                interpolation_started_with_curly_level: 42,
                metadata: StringLiteralMetadata::Plain
            })
        )
    }
);
assert_lex!(
    test_tSYMBEG,
    ":'",
    tSYMBEG,
    0..2,
    setup = |lexer: &mut Lexer| {
        lexer.curly_nest = 42;
    },
    assert = |lexer: &Lexer| {
        assert_eq!(lexer.string_literals.size(), 1);
        assert_eq!(
            lexer.string_literals.last(),
            Some(StringLiteral {
                supports_interpolation: false,
                currently_in_interpolation: false,
                ends_with: b" ",
                interpolation_started_with_curly_level: 0,
                metadata: StringLiteralMetadata::Plain
            })
        )
    }
);
assert_lex!(test_tCOLON, ":", tCOLON, 0..1);

impl<'a> OnByte<'a, b'/'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        if self.new_expr_required {
            let token = Token(TokenValue::tREGEXP_BEG(b"/"), Loc(start, self.pos()));
            self.string_literals.push(StringLiteral {
                supports_interpolation: true,
                currently_in_interpolation: false,
                ends_with: b"/",
                interpolation_started_with_curly_level: self.curly_nest,
                metadata: StringLiteralMetadata::Plain,
            });
            return token;
        }

        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                Token(TokenValue::tOP_ASGN(b"/="), Loc(start, self.pos()))
            }
            _ => Token(TokenValue::tDIVIDE, Loc(start, self.pos())),
        }
    }
}
assert_lex!(
    test_tREGEXP_BEG,
    "/",
    tREGEXP_BEG(b"/"),
    0..1,
    setup = |lexer: &mut Lexer| {
        lexer.require_new_expr();
        lexer.curly_nest = 42;
    },
    assert = |lexer: &Lexer| {
        assert_eq!(lexer.string_literals.size(), 1);
        assert_eq!(
            lexer.string_literals.last(),
            Some(StringLiteral {
                supports_interpolation: true,
                currently_in_interpolation: false,
                ends_with: b"/",
                interpolation_started_with_curly_level: 42,
                metadata: StringLiteralMetadata::Plain
            })
        )
    }
);
assert_lex!(test_tOP_ASGN_DIV, "/=", tOP_ASGN(b"/="), 0..2);
assert_lex!(test_tDIVIDE, "/", tDIVIDE, 0..1);

impl<'a> OnByte<'a, b'^'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();

        match self.current_byte() {
            Some(b'=') => {
                self.skip_byte();
                Token(TokenValue::tOP_ASGN(b"^="), Loc(start, self.pos()))
            }
            _ => Token(TokenValue::tCARET, Loc(start, self.pos())),
        }
    }
}
assert_lex!(test_tOP_ASGN_CARET, "^=", tOP_ASGN(b"^="), 0..2);
assert_lex!(test_tCARET, "^", tCARET, 0..1);

impl<'a> OnByte<'a, b';'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        Token(TokenValue::tSEMI, Loc(start, self.pos()))
    }
}
assert_lex!(test_tSEMI, ";", tSEMI, 0..1);

impl<'a> OnByte<'a, b','> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        Token(TokenValue::tCOMMA, Loc(start, self.pos()))
    }
}
assert_lex!(test_tCOMMA, ",", tCOMMA, 0..1);

impl<'a> OnByte<'a, b'~'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        // ~@ is handled on the parser level
        Token(TokenValue::tTILDE, Loc(start, self.pos()))
    }
}
assert_lex!(test_tTILDE, "~", tTILDE, 0..1);

impl<'a> OnByte<'a, b'('> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        self.paren_nest += 1;
        Token(TokenValue::tLPAREN, Loc(start, self.pos()))
    }
}
assert_lex!(test_tLPAREN, "(", tLPAREN, 0..1);

impl<'a> OnByte<'a, b'['> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        self.brack_nest += 1;
        Token(TokenValue::tLBRACK, Loc(start, self.pos()))
    }
}
assert_lex!(test_tLBRACK, "[", tLBRACK, 0..1);

impl<'a> OnByte<'a, b'{'> for Lexer<'a> {
    fn on_byte(&mut self) -> Token<'a> {
        let start = self.pos();
        self.skip_byte();
        self.curly_nest += 1;
        Token(TokenValue::tLCURLY, Loc(start, self.pos()))
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
                Token(TokenValue::tSP, Loc(start, self.pos()))
            }
            Some(b'\t') => {
                self.skip_byte();
                Token(TokenValue::tSLASH_T, Loc(start, self.pos()))
            }
            Some(0x0c) => {
                self.skip_byte();
                Token(TokenValue::tSLASH_F, Loc(start, self.pos()))
            }
            Some(b'\r') => {
                self.skip_byte();
                Token(TokenValue::tSLASH_R, Loc(start, self.pos()))
            }
            Some(0x0b) => {
                self.skip_byte();
                Token(TokenValue::tVTAB, Loc(start, self.pos()))
            }
            _ => Token(TokenValue::tBACKSLASH, Loc(start, self.pos())),
        }
    }
}
assert_lex!(test_tESCAPED_NL, "\\\nTEST_TOKEN", tTEST_TOKEN, 2..12);
assert_lex!(test_tESCAPED_SP, "\\ ", tSP, 0..2);
assert_lex!(test_tESCAPED_TAB, "\\\t", tSLASH_T, 0..2);
assert_lex!(test_tESCAPED_LF, "\\\x0c", tSLASH_F, 0..2);
assert_lex!(test_tESCAPED_CR, "\\\r", tSLASH_R, 0..2);
assert_lex!(test_tESCAPED_VTAB, "\\\x0b", tVTAB, 0..2);

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
            None | Some(b'\n') if self.buffer.const_lookahead(b"__END__") => {
                return Token(TokenValue::tEOF, Loc(start, start));
            }
            _ => {}
        }

        self.buffer.set_pos(start);
        parse_ident(&mut self.buffer)
    }
}
assert_lex!(test_tEOF_at__END__, "__END__", tEOF, 0..0);
assert_lex!(test_tEOF_at_NL___END__, "\n__END__", tEOF, 1..1);
