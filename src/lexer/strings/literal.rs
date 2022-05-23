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
    pub(crate) next_action: NextAction<'a>,

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum StringExtendAction<'a> {
    EmitToken { token: Token<'a> },
    FoundStringEnd { token: Token<'a> },
    FoundInterpolation { token: Token<'a> },
    NoAction,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum NextAction<'a> {
    NoAction,
    OneAction(StringExtendAction<'a>),
    TwoActions(StringExtendAction<'a>, StringExtendAction<'a>),
}

impl<'a> NextAction<'a> {
    fn take(&mut self) -> Option<StringExtendAction<'a>> {
        match *self {
            Self::NoAction => None,
            Self::OneAction(action) => {
                *self = Self::NoAction;
                Some(action)
            }
            Self::TwoActions(first, second) => {
                *self = Self::OneAction(second);
                Some(first)
            }
        }
    }

    fn add(&mut self, action: StringExtendAction<'a>) {
        match self {
            Self::NoAction => {
                *self = Self::OneAction(action);
            }
            Self::OneAction(first) => {
                *self = Self::TwoActions(*first, action);
            }
            Self::TwoActions(_, _) => {
                unreachable!("The queue of string extend actions supports only 2 elements")
            }
        }
    }

    fn is_empty(&self) -> bool {
        matches!(self, Self::NoAction)
    }
}

impl Default for NextAction<'_> {
    fn default() -> Self {
        Self::NoAction
    }
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

    pub(crate) fn is_currently_in_interpolation(&self) -> bool {
        if !self.next_action.is_empty() {
            // There's still work to emit before making a decision on interpolation
            return false;
        }

        self.supports_interpolation && self.currently_in_interpolation
    }

    pub(crate) fn can_close_interpolation(
        &self,
        buffer: &mut Buffer<'a>,
        curly_nest: usize,
    ) -> bool {
        if !self.next_action.is_empty() {
            // There's still work to emit before making a decision on interpolation
            return false;
        }

        buffer.current_byte() == Some(b'}')
            && self.interpolation_started_with_curly_level == curly_nest
    }

    pub(crate) fn extend(&mut self, buffer: &mut Buffer<'a>) -> StringExtendAction<'a> {
        if let Some(cached_action) = self.next_action.take() {
            return cached_action;
        }

        let start = buffer.pos();

        // Utility helper: checks whether there's recorded string content,
        // returns a tSTRING_CONTENT is there's any
        fn string_content_to_emit<'a>(
            buffer: &mut Buffer<'a>,
            start: usize,
            end: usize,
        ) -> Option<Token<'a>> {
            if start == end {
                None
            } else {
                Some(Token(
                    TokenValue::tSTRING_CONTENT(buffer.slice(start, end)),
                    Loc(start, end),
                ))
            }
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
                if buffer.current_byte().is_none() {
                    if let Some(token) = string_content_to_emit(buffer, start, buffer.pos()) {
                        return StringExtendAction::EmitToken { token };
                    } else {
                        return StringExtendAction::NoAction;
                    }
                }

                if buffer.const_lookahead(b"#{") {
                    // #{ interpolation
                    let action = StringExtendAction::FoundInterpolation {
                        token: Token(
                            TokenValue::tSTRING_DBEG,
                            Loc(buffer.pos(), buffer.pos() + 2),
                        ),
                    };
                    let string_content = string_content_to_emit(buffer, start, buffer.pos());
                    buffer.set_pos(buffer.pos() + 2);

                    if let Some(token) = string_content {
                        self.next_action.add(action);
                        return StringExtendAction::EmitToken { token };
                    } else {
                        return action;
                    }
                }

                if buffer.const_lookahead(b"#@@") {
                    if let Some(ident_end) = read_ident(buffer, buffer.pos() + 3) {
                        // #@@foo interpolation
                        let interp_action = StringExtendAction::EmitToken {
                            token: Token(
                                TokenValue::tSTRING_DVAR,
                                Loc(buffer.pos(), buffer.pos() + 1),
                            ),
                        };
                        let var_action = StringExtendAction::EmitToken {
                            token: Token(
                                TokenValue::tCVAR(buffer.slice(buffer.pos() + 1, ident_end)),
                                Loc(buffer.pos() + 1, ident_end),
                            ),
                        };
                        let string_content = string_content_to_emit(buffer, start, buffer.pos());
                        buffer.set_pos(ident_end);

                        if let Some(token) = string_content {
                            self.next_action.add(interp_action);
                            self.next_action.add(var_action);
                            return StringExtendAction::EmitToken { token };
                        } else {
                            self.next_action.add(var_action);
                            return interp_action;
                        }
                    } else {
                        // just #@@ string content without subsequent identifier
                        // keep reading
                    }
                }
                if buffer.const_lookahead(b"#@") {
                    if let Some(ident_end) = read_ident(buffer, buffer.pos() + 2) {
                        // #@foo interpolation
                        let interp_action = StringExtendAction::EmitToken {
                            token: Token(
                                TokenValue::tSTRING_DVAR,
                                Loc(buffer.pos(), buffer.pos() + 1),
                            ),
                        };
                        let var_action = StringExtendAction::EmitToken {
                            token: Token(
                                TokenValue::tIVAR(buffer.slice(buffer.pos() + 1, ident_end)),
                                Loc(buffer.pos() + 1, ident_end),
                            ),
                        };
                        let string_content = string_content_to_emit(buffer, start, buffer.pos());
                        buffer.set_pos(ident_end);

                        if let Some(token) = string_content {
                            self.next_action.add(interp_action);
                            self.next_action.add(var_action);
                            return StringExtendAction::EmitToken { token };
                        } else {
                            self.next_action.add(var_action);
                            return interp_action;
                        }
                    } else {
                        // just #@ string content without subsequent identifier
                        // keep reading
                    }
                }
                if buffer.const_lookahead(b"#$") {
                    if let Some(ident_end) = read_ident(buffer, buffer.pos() + 2) {
                        // #$foo interpolation
                        let interp_action = StringExtendAction::EmitToken {
                            token: Token(
                                TokenValue::tSTRING_DVAR,
                                Loc(buffer.pos(), buffer.pos() + 1),
                            ),
                        };
                        let var_action = StringExtendAction::EmitToken {
                            token: Token(
                                TokenValue::tGVAR(buffer.slice(buffer.pos() + 1, ident_end)),
                                Loc(buffer.pos() + 1, ident_end),
                            ),
                        };
                        let string_content = string_content_to_emit(buffer, start, buffer.pos());
                        buffer.set_pos(ident_end);

                        if let Some(token) = string_content {
                            self.next_action.add(interp_action);
                            self.next_action.add(var_action);
                            return StringExtendAction::EmitToken { token };
                        } else {
                            self.next_action.add(var_action);
                            return interp_action;
                        }
                    } else {
                        // just #$ string content without subsequent identifier
                        // keep reading
                    }
                }
                if buffer.lookahead(self.ends_with) {
                    let string_end_action = StringExtendAction::FoundStringEnd {
                        token: Token(
                            TokenValue::tSTRING_END(self.ends_with),
                            Loc(buffer.pos(), buffer.pos() + self.ends_with.len()),
                        ),
                    };
                    let string_content = string_content_to_emit(buffer, start, buffer.pos());
                    buffer.set_pos(buffer.pos() + self.ends_with.len());

                    if let Some(token) = string_content {
                        self.next_action.add(string_end_action);
                        return StringExtendAction::EmitToken { token };
                    } else {
                        return string_end_action;
                    }
                }
                if buffer.const_lookahead(b"\\\n") {
                    // just emit what we've got so far
                    // parser will merge two consectuive string literals
                    let action = StringExtendAction::EmitToken {
                        token: Token(
                            TokenValue::tSTRING_CONTENT(buffer.slice(start, buffer.pos())),
                            Loc(start, buffer.pos()),
                        ),
                    };
                    // and skip escaped NL
                    buffer.set_pos(buffer.pos() + 2);
                    return action;
                }
                buffer.skip_byte();
            }
        } else {
            loop {
                if buffer.current_byte().is_none() {
                    if let Some(token) = string_content_to_emit(buffer, start, buffer.pos()) {
                        return StringExtendAction::EmitToken { token };
                    } else {
                        return StringExtendAction::NoAction;
                    }
                }
                if buffer.lookahead(self.ends_with) {
                    let string_end_action = StringExtendAction::FoundStringEnd {
                        token: Token(
                            TokenValue::tSTRING_END(self.ends_with),
                            Loc(buffer.pos(), buffer.pos() + self.ends_with.len()),
                        ),
                    };
                    let string_content = string_content_to_emit(buffer, start, buffer.pos());
                    buffer.set_pos(buffer.pos() + self.ends_with.len());

                    if let Some(token) = string_content {
                        self.next_action.add(string_end_action);
                        return StringExtendAction::EmitToken { token };
                    } else {
                        return string_end_action;
                    }
                }
                buffer.skip_byte();
            }
        }
    }
}
