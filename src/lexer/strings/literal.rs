use crate::{
    lexer::{buffer::Buffer, ident::is_identchar},
    token::{Loc, Token, TokenValue},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct StringLiteral<'a> {
    pub(crate) supports_interpolation: bool,
    pub(crate) currently_in_interpolation: bool,
    pub(crate) ends_with: &'a [u8],
    pub(crate) interpolation_started_with_curly_level: usize,
    pub(crate) next_token: Option<Token<'a>>,

    pub(crate) metadata: StringLiteralMetadata,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum StringLiteralMetadata {
    String,
    Symbol,
    Regexp,
    Heredoc { heredoc_id_ended_at: usize },
}

impl Default for StringLiteralMetadata {
    fn default() -> Self {
        Self::String
    }
}

#[derive(Debug)]
pub(crate) enum StringExtendAction<'a> {
    FoundStringEnd {
        string_end_starts_at: usize,
    },
    FoundInterpolation {
        interpolation_starts_at: usize,
    },
    FoundInterpolatedToken {
        interp_token: Token<'a>,
        var_token: Token<'a>,
    },
    FoundEscapedNl {
        escaped_nl_starts_at: usize,
    },
}

impl<'a> StringLiteral<'a> {
    pub(crate) fn string() -> Self {
        Self {
            metadata: StringLiteralMetadata::String,
            ..Self::default()
        }
    }
    pub(crate) fn symbol() -> Self {
        Self {
            metadata: StringLiteralMetadata::Symbol,
            ..Self::default()
        }
    }
    pub(crate) fn regexp() -> Self {
        Self {
            metadata: StringLiteralMetadata::Regexp,
            ..Self::default()
        }
    }
    pub(crate) fn heredoc(heredoc_id_ended_at: usize) -> Self {
        Self {
            metadata: StringLiteralMetadata::Heredoc {
                heredoc_id_ended_at,
            },
            ..Self::default()
        }
    }

    pub(crate) fn with_interpolation_support(mut self, value: bool) -> Self {
        self.supports_interpolation = value;
        self
    }
    pub(crate) fn with_ending(mut self, value: &'a [u8]) -> Self {
        self.ends_with = value;
        self
    }
    pub(crate) fn with_curly_level(mut self, value: usize) -> Self {
        self.interpolation_started_with_curly_level = value;
        self
    }

    pub(crate) fn extend(&self, buffer: &mut Buffer<'a>) -> StringExtendAction<'a> {
        // Check if it's a string end
        if buffer.lookahead(self.ends_with) {
            return StringExtendAction::FoundStringEnd {
                string_end_starts_at: buffer.pos(),
            };
        }

        // otherwise it's just a string content:
        // 1. for interpolation: until
        //      1. "#{"
        //      2. "#@@<identchar>"
        //      3. "#@<identchar>"
        //      4. "#$<identchar>"
        //      5. string end
        //      6. escaped tNL
        // 2. for non-interpolation - until string end
        if self.supports_interpolation {
            loop {
                if buffer.const_lookahead(b"#{") {
                    return StringExtendAction::FoundInterpolation {
                        interpolation_starts_at: buffer.pos(),
                    };
                }

                fn read_ident<'a>(buffer: &mut Buffer<'a>, start: usize) -> Option<usize> {
                    let mut end = start;
                    while buffer.byte_at(end).map(|byte| is_identchar(byte)) == Some(true) {
                        end += 1;
                    }
                    if start != end && !matches!(buffer.byte_at(start), Some(b'0'..=b'9')) {
                        Some(end)
                    } else {
                        None
                    }
                }

                if buffer.const_lookahead(b"#@@") {
                    if let Some(ident_end) = read_ident(buffer, buffer.pos() + 3) {
                        // #@@foo interpolation
                        return StringExtendAction::FoundInterpolatedToken {
                            interp_token: Token(
                                TokenValue::tSTRING_DVAR,
                                Loc(buffer.pos(), buffer.pos() + 1),
                            ),
                            var_token: Token(
                                TokenValue::tCVAR(buffer.slice(buffer.pos() + 1, ident_end)),
                                Loc(buffer.pos() + 1, ident_end),
                            ),
                        };
                    } else {
                        // just #@@ string content
                        // keep reading
                    }
                }
                if buffer.const_lookahead(b"#@") {
                    if let Some(ident_end) = read_ident(buffer, buffer.pos() + 2) {
                        // #@foo interpolation
                        return StringExtendAction::FoundInterpolatedToken {
                            interp_token: Token(
                                TokenValue::tSTRING_DVAR,
                                Loc(buffer.pos(), buffer.pos() + 1),
                            ),
                            var_token: Token(
                                TokenValue::tIVAR(buffer.slice(buffer.pos() + 1, ident_end)),
                                Loc(buffer.pos() + 1, ident_end),
                            ),
                        };
                    } else {
                        // just #@ string content
                        // keep reading
                    }
                }
                if buffer.const_lookahead(b"#$") {
                    if let Some(ident_end) = read_ident(buffer, buffer.pos() + 2) {
                        // #@foo interpolation
                        return StringExtendAction::FoundInterpolatedToken {
                            interp_token: Token(
                                TokenValue::tSTRING_DVAR,
                                Loc(buffer.pos(), buffer.pos() + 1),
                            ),
                            var_token: Token(
                                TokenValue::tGVAR(buffer.slice(buffer.pos() + 1, ident_end)),
                                Loc(buffer.pos() + 1, ident_end),
                            ),
                        };
                    } else {
                        // just #$ string content
                        // keep reading
                    }
                }
                if buffer.lookahead(self.ends_with) {
                    return StringExtendAction::FoundStringEnd {
                        string_end_starts_at: buffer.pos(),
                    };
                }
                if buffer.const_lookahead(b"\\\n") {
                    return StringExtendAction::FoundEscapedNl {
                        escaped_nl_starts_at: buffer.pos(),
                    };
                }
                buffer.skip_byte();
            }
        } else {
            loop {
                if buffer.lookahead(self.ends_with) {
                    return StringExtendAction::FoundStringEnd {
                        string_end_starts_at: buffer.pos(),
                    };
                }
                buffer.skip_byte();
            }
        }
    }
}
