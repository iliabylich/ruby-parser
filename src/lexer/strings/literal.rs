use crate::{
    lexer::{buffer::Buffer, ident::is_identchar},
    token::{Loc, Token, TokenValue},
};

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
pub(crate) enum StringExtendAction<'a> {
    FoundStringEnd { string_end_starts_at: usize },
    FoundInterpolation { interpolation_starts_at: usize },
    FoundInterpolatedToken { token: Token<'a> },
    FoundEscapedNl { escaped_nl_starts_at: usize },
}

impl<'a> StringLiteral<'a> {
    pub(crate) fn new(
        supports_interpolation: bool,
        ends_with: &'a [u8],
        interpolation_started_with_curly_level: usize,
        metadata: StringLiteralMetadata,
    ) -> Self {
        Self {
            supports_interpolation,
            currently_in_interpolation: false,
            ends_with,
            interpolation_started_with_curly_level,
            metadata,
        }
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
                if buffer.const_lookahead(b"#@@") {
                    // FIXME: this is an absolutely incorrect stub
                    return StringExtendAction::FoundInterpolatedToken {
                        token: Token(TokenValue::tCVAR(b"@@foo"), Loc(1, 2)),
                    };
                }
                if buffer.const_lookahead(b"#@") {
                    // FIXME: this is an absolutely incorrect stub
                    return StringExtendAction::FoundInterpolatedToken {
                        token: Token(TokenValue::tIVAR(b"@foo"), Loc(1, 2)),
                    };
                }
                if buffer.const_lookahead(b"#$") {
                    // FIXME: this is an absolutely incorrect stub
                    return StringExtendAction::FoundInterpolatedToken {
                        token: Token(TokenValue::tGVAR(b"$foo"), Loc(1, 2)),
                    };
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
