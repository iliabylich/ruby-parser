use crate::{
    lexer::buffer::Buffer,
    token::{Loc, Token, TokenValue},
};

pub(crate) struct StringLiteralStack<'a> {
    stack: Vec<StringLiteral<'a>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum StringLiteral<'a> {
    Plain {
        supports_interpolation: bool,
        currently_in_interpolation: bool,
        ends_with: &'a [u8],
        interpolation_started_with_curly_level: usize,
    },

    Heredoc {
        supports_interpolation: bool,
        currently_in_interpolation: bool,
        ends_with: &'a [u8],
        heredoc_id_ended_at: usize,
        interpolation_started_with_curly_level: usize,
    },
}

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

    pub(crate) fn size(&self) -> usize {
        self.stack.len()
    }
}

enum ExtendAction<'a> {
    FoundStringEnd { string_end_starts_at: usize },
    FoundInterpolation { interpolation_starts_at: usize },
    FoundInterpolatedToken { token: Token<'a> },
    FoundEscapedNl { escaped_nl_starts_at: usize },
}

impl<'a> StringLiteral<'a> {
    fn supports_interpolation(&self) -> bool {
        match self {
            StringLiteral::Plain {
                supports_interpolation,
                ..
            } => *supports_interpolation,
            StringLiteral::Heredoc {
                supports_interpolation,
                ..
            } => *supports_interpolation,
        }
    }

    fn currently_in_interpolation(&self) -> bool {
        match self {
            StringLiteral::Plain {
                currently_in_interpolation,
                ..
            } => *currently_in_interpolation,
            StringLiteral::Heredoc {
                currently_in_interpolation,
                ..
            } => *currently_in_interpolation,
        }
    }

    fn currently_in_interpolation_mut(&mut self) -> &mut bool {
        match self {
            StringLiteral::Plain {
                currently_in_interpolation,
                ..
            } => currently_in_interpolation,
            StringLiteral::Heredoc {
                currently_in_interpolation,
                ..
            } => currently_in_interpolation,
        }
    }

    fn ends_with(&self) -> &'a [u8] {
        match self {
            StringLiteral::Plain { ends_with, .. } => *ends_with,
            StringLiteral::Heredoc { ends_with, .. } => *ends_with,
        }
    }

    fn interpolation_started_with_curly_level(&self) -> usize {
        match self {
            StringLiteral::Plain {
                interpolation_started_with_curly_level,
                ..
            } => *interpolation_started_with_curly_level,
            StringLiteral::Heredoc {
                interpolation_started_with_curly_level,
                ..
            } => *interpolation_started_with_curly_level,
        }
    }

    pub(crate) fn stop_interpolation(&mut self) {
        match self {
            StringLiteral::Plain {
                currently_in_interpolation,
                ..
            } => *currently_in_interpolation = false,
            StringLiteral::Heredoc {
                currently_in_interpolation,
                ..
            } => *currently_in_interpolation = false,
        }
    }

    pub(crate) fn lex(&mut self, buffer: &mut Buffer<'a>) -> StringLiteralAction<'a> {
        if self.currently_in_interpolation() && self.supports_interpolation() {
            // yield control to lexer to read interpolated tokens
            return StringLiteralAction::ReadInterpolatedContent {
                interpolation_started_with_curly_level: self
                    .interpolation_started_with_curly_level(),
            };
        }

        let start = buffer.pos();

        match self.try_to_extend(buffer) {
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
                        TokenValue::tSTRING_END(self.ends_with()),
                        Loc(
                            string_end_starts_at,
                            string_end_starts_at + self.ends_with().len(),
                        ),
                    );
                    // Set buffer.pos to post-string location
                    buffer.set_pos(string_end_starts_at + self.ends_with().len());
                    StringLiteralAction::CloseLiteral { end_token: token }
                }
            }
            ExtendAction::FoundInterpolation {
                interpolation_starts_at,
            } => {
                let in_interpolation = self.currently_in_interpolation_mut();
                *in_interpolation = true;

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
        if buffer.lookahead(self.ends_with()) {
            return ExtendAction::FoundStringEnd {
                string_end_starts_at: buffer.pos() + self.ends_with().len(),
            };
        }

        let start = buffer.pos();

        // is it interpolation?
        if self.supports_interpolation() {
            match buffer.byte_at(start) {
                Some(b'#') => {
                    // potentially an interpolation
                    match buffer.byte_at(start + 1) {
                        Some(b'{') => {
                            // #{ interpolation
                            return ExtendAction::FoundInterpolation {
                                interpolation_starts_at: start + 1,
                            };
                        }
                        Some(b'@') => {
                            // #@@x or #@x or #$x
                            // FIXME: this is just a stub that is 100% incorrect
                            return ExtendAction::FoundInterpolatedToken {
                                token: Token(TokenValue::tIVAR(b"@foo"), Loc(1, 2)),
                            };
                        }
                        _ => {
                            // no inerpolation
                        }
                    }
                }
                _ => {
                    // no interpolation
                }
            }
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
        if self.supports_interpolation() {
            loop {
                if buffer.lookahead(b"#{") {
                    return ExtendAction::FoundInterpolation {
                        interpolation_starts_at: buffer.pos(),
                    };
                }
                if buffer.lookahead(b"#@@") {
                    // FIXME: this is an absolutely incorrect stub
                    return ExtendAction::FoundInterpolatedToken {
                        token: Token(TokenValue::tCVAR(b"@@foo"), Loc(1, 2)),
                    };
                }
                if buffer.lookahead(b"#@") {
                    // FIXME: this is an absolutely incorrect stub
                    return ExtendAction::FoundInterpolatedToken {
                        token: Token(TokenValue::tIVAR(b"@foo"), Loc(1, 2)),
                    };
                }
                if buffer.lookahead(b"#$") {
                    // FIXME: this is an absolutely incorrect stub
                    return ExtendAction::FoundInterpolatedToken {
                        token: Token(TokenValue::tGVAR(b"$foo"), Loc(1, 2)),
                    };
                }
                if buffer.lookahead(self.ends_with()) {
                    return ExtendAction::FoundStringEnd {
                        string_end_starts_at: buffer.pos(),
                    };
                }
                if buffer.lookahead(b"\\\n") {
                    return ExtendAction::FoundEscapedNl {
                        escaped_nl_starts_at: buffer.pos(),
                    };
                }
                buffer.skip_byte();
            }
        } else {
            loop {
                if buffer.lookahead(self.ends_with()) {
                    return ExtendAction::FoundStringEnd {
                        string_end_starts_at: buffer.pos(),
                    };
                }
                buffer.skip_byte();
            }
        }
    }
}
