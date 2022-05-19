use crate::{
    lexer::buffer::Buffer,
    token::{Loc, Token, TokenValue},
};

pub(crate) struct StringLiteralStack<'a> {
    stack: Vec<StringLiteral<'a>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct StringLiteral<'a> {
    pub(crate) supports_interpolation: bool,
    pub(crate) currently_in_interpolation: bool,
    pub(crate) ends_with: &'a [u8],
    pub(crate) interpolation_started_with_curly_level: usize,

    pub(crate) metadata: StringLiteralMetadata,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum StringLiteralMetadata {
    Plain,
    Heredoc { heredoc_id_ended_at: usize },
}

#[derive(Debug)]
pub(crate) enum StringLiteralAction<'a> {
    ReadInterpolatedContent {
        interpolation_started_with_curly_level: usize,
    },
    EmitToken {
        token: Token<'a>,
    },
    CloseLiteral {
        end_token: Token<'a>,
    },
}

impl<'a> StringLiteralStack<'a> {
    pub(crate) fn new() -> Self {
        Self { stack: vec![] }
    }

    pub(crate) fn last(&self) -> Option<StringLiteral<'a>> {
        self.stack.last().map(|literal| *literal)
    }

    pub(crate) fn last_mut(&mut self) -> Option<&mut StringLiteral<'a>> {
        self.stack.last_mut()
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop().unwrap();
    }

    pub(crate) fn push(&mut self, literal: StringLiteral<'a>) {
        self.stack.push(literal);
    }

    #[cfg(test)]
    pub(crate) fn size(&self) -> usize {
        self.stack.len()
    }
}

#[derive(Debug)]
enum ExtendAction<'a> {
    FoundStringEnd { string_end_starts_at: usize },
    FoundInterpolation { interpolation_starts_at: usize },
    FoundInterpolatedToken { token: Token<'a> },
    FoundEscapedNl { escaped_nl_starts_at: usize },
}

impl<'a> StringLiteral<'a> {
    pub(crate) fn stop_interpolation(&mut self) {
        self.currently_in_interpolation = false;
    }

    pub(crate) fn lex(&mut self, buffer: &mut Buffer<'a>) -> StringLiteralAction<'a> {
        if self.currently_in_interpolation && self.supports_interpolation {
            // yield control to lexer to read interpolated tokens
            return StringLiteralAction::ReadInterpolatedContent {
                interpolation_started_with_curly_level: self.interpolation_started_with_curly_level,
            };
        }

        let start = buffer.pos();

        let extend_action = self.try_to_extend(buffer);
        match extend_action {
            ExtendAction::FoundStringEnd {
                string_end_starts_at,
            } => {
                // flush what's available (if any)
                if string_end_starts_at > start {
                    let token = Token(
                        TokenValue::tSTRING_CONTENT(buffer.slice(start, string_end_starts_at)),
                        Loc(start, string_end_starts_at),
                    );
                    // Set buffer.pos to tSTRING_END loc that will be recorded
                    // on the next run
                    buffer.set_pos(string_end_starts_at);
                    StringLiteralAction::EmitToken { token }
                } else {
                    // No string content recorded, just emit tSTRING_END
                    let token = Token(
                        TokenValue::tSTRING_END(self.ends_with),
                        Loc(
                            string_end_starts_at,
                            string_end_starts_at + self.ends_with.len(),
                        ),
                    );
                    // Set buffer.pos to post-string location
                    buffer.set_pos(string_end_starts_at + self.ends_with.len());
                    StringLiteralAction::CloseLiteral { end_token: token }
                }
            }
            ExtendAction::FoundInterpolation {
                interpolation_starts_at,
            } => {
                // flush what's available (if any)
                if interpolation_starts_at > start {
                    let token = Token(
                        TokenValue::tSTRING_CONTENT(buffer.slice(start, interpolation_starts_at)),
                        Loc(start, interpolation_starts_at),
                    );
                    // Set buffer.pos to tSTRING_END loc that will be recorded
                    // on the next run
                    buffer.set_pos(interpolation_starts_at);
                    StringLiteralAction::EmitToken { token }
                } else {
                    // No string content recorded
                    let token = Token(
                        TokenValue::tSTRING_DBEG(b"#{"),
                        Loc(interpolation_starts_at, interpolation_starts_at + 2),
                    );
                    self.currently_in_interpolation = true;
                    buffer.set_pos(interpolation_starts_at + 2);
                    StringLiteralAction::EmitToken { token }
                }
            }
            ExtendAction::FoundInterpolatedToken { token } => {
                buffer.set_pos(token.loc().end());
                StringLiteralAction::EmitToken { token }
            }
            ExtendAction::FoundEscapedNl {
                escaped_nl_starts_at,
            } => {
                let token = Token(
                    TokenValue::tSTRING_CONTENT(buffer.slice(start, escaped_nl_starts_at)),
                    Loc(start, escaped_nl_starts_at),
                );
                // Set buffer.pos to tSTRING_END loc that will be recorded
                // on the next run
                buffer.set_pos(escaped_nl_starts_at);
                StringLiteralAction::EmitToken { token }
            }
        }
    }

    fn try_to_extend(&self, buffer: &mut Buffer<'a>) -> ExtendAction<'a> {
        // Check if it's a string end
        if buffer.lookahead(self.ends_with) {
            return ExtendAction::FoundStringEnd {
                string_end_starts_at: buffer.pos(),
            };
        }

        // otherwise it's just a string content:
        // 1. for interpolation: until
        //      1. "#{"
        //      2. "#@@x"
        //      3. "#@x"
        //      4. "#$x"
        //      5. string end
        //      6. escaped tNL
        // 2. for non-interpolation - until string end
        if self.supports_interpolation {
            loop {
                if buffer.const_lookahead(b"#{") {
                    return ExtendAction::FoundInterpolation {
                        interpolation_starts_at: buffer.pos(),
                    };
                }
                if buffer.const_lookahead(b"#@@") {
                    // FIXME: this is an absolutely incorrect stub
                    return ExtendAction::FoundInterpolatedToken {
                        token: Token(TokenValue::tCVAR(b"@@foo"), Loc(1, 2)),
                    };
                }
                if buffer.const_lookahead(b"#@") {
                    // FIXME: this is an absolutely incorrect stub
                    return ExtendAction::FoundInterpolatedToken {
                        token: Token(TokenValue::tIVAR(b"@foo"), Loc(1, 2)),
                    };
                }
                if buffer.const_lookahead(b"#$") {
                    // FIXME: this is an absolutely incorrect stub
                    return ExtendAction::FoundInterpolatedToken {
                        token: Token(TokenValue::tGVAR(b"$foo"), Loc(1, 2)),
                    };
                }
                if buffer.lookahead(self.ends_with) {
                    return ExtendAction::FoundStringEnd {
                        string_end_starts_at: buffer.pos(),
                    };
                }
                if buffer.const_lookahead(b"\\\n") {
                    return ExtendAction::FoundEscapedNl {
                        escaped_nl_starts_at: buffer.pos(),
                    };
                }
                buffer.skip_byte();
            }
        } else {
            loop {
                if buffer.lookahead(self.ends_with) {
                    return ExtendAction::FoundStringEnd {
                        string_end_starts_at: buffer.pos(),
                    };
                }
                buffer.skip_byte();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        token::{Loc, Token, TokenValue},
    };

    #[test]
    fn test_string_plain_non_interp() {
        let mut lexer = Lexer::new("'foo'");
        assert_eq!(
            lexer.tokenize_until_eof(),
            vec![
                Token(TokenValue::tSTRING_BEG(b"'"), Loc(0, 1)),
                Token(TokenValue::tSTRING_CONTENT(b"foo"), Loc(1, 4)),
                Token(TokenValue::tSTRING_END(b"'"), Loc(4, 5)),
                Token(TokenValue::tEOF, Loc(5, 5))
            ]
        );
    }

    #[test]
    fn test_string_plain_interp() {
        let mut lexer = Lexer::new("\"foo#{TEST_TOKEN}bar\"");
        assert_eq!(
            lexer.tokenize_until_eof(),
            vec![
                Token(TokenValue::tSTRING_BEG(b"\""), Loc(0, 1)),
                Token(TokenValue::tSTRING_CONTENT(b"foo"), Loc(1, 4)),
                Token(TokenValue::tSTRING_DBEG(b"#{"), Loc(4, 6)),
                Token(TokenValue::tTEST_TOKEN, Loc(6, 16)),
                Token(TokenValue::tSTRING_DEND, Loc(16, 17)),
                Token(TokenValue::tSTRING_CONTENT(b"bar"), Loc(17, 20)),
                Token(TokenValue::tSTRING_END(b"\""), Loc(20, 21)),
                Token(TokenValue::tEOF, Loc(21, 21))
            ]
        );
    }

    #[test]
    fn test_string_interp_braces() {
        let mut lexer = Lexer::new("\"#{{} + {}}\"");
        assert_eq!(
            lexer.tokenize_until_eof(),
            vec![
                Token(TokenValue::tSTRING_BEG(b"\""), Loc(0, 1)),
                Token(TokenValue::tSTRING_DBEG(b"#{"), Loc(1, 3)),
                Token(TokenValue::tLCURLY, Loc(3, 4)),
                Token(TokenValue::tRCURLY, Loc(4, 5)),
                Token(TokenValue::tPLUS, Loc(6, 7)),
                Token(TokenValue::tLCURLY, Loc(8, 9)),
                Token(TokenValue::tRCURLY, Loc(9, 10)),
                Token(TokenValue::tSTRING_DEND, Loc(10, 11)),
                Token(TokenValue::tSTRING_END(b"\""), Loc(11, 12)),
                Token(TokenValue::tEOF, Loc(12, 12)),
            ]
        );
    }
}
