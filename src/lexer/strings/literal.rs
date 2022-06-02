use std::ops::ControlFlow;

use crate::{
    lexer::{
        atmark::{lookahead_atmark, LookaheadAtMarkResult},
        buffer::Buffer,
        gvar::{lookahead_gvar, LookaheadGvarResult},
    },
    token::{Loc, Token, TokenValue},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct StringLiteral<'a> {
    pub(crate) supports_interpolation: bool,
    pub(crate) currently_in_interpolation: bool,
    pub(crate) ends_with: &'a [u8],
    pub(crate) interpolation_started_with_curly_level: usize,
    pub(crate) next_action: NextAction,

    pub(crate) metadata: StringLiteralMetadata,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum StringLiteralMetadata {
    String,
    Symbol,
    Regexp,
    Array,
    Heredoc { heredoc_id_ended_at: usize },
}

impl Default for StringLiteralMetadata {
    fn default() -> Self {
        Self::String
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum StringExtendAction {
    EmitToken { token: Token },
    FoundStringEnd { token: Token },
    FoundInterpolation { token: Token },
    EmitEOF,
    ReadInterpolatedContent,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum NextAction {
    NoAction,
    OneAction(StringExtendAction),
    TwoActions(StringExtendAction, StringExtendAction),
}

impl<'a> NextAction {
    fn take(&mut self) -> Option<StringExtendAction> {
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

    fn add(&mut self, action: StringExtendAction) {
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
}

impl Default for NextAction {
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
    pub(crate) fn array() -> Self {
        Self {
            metadata: StringLiteralMetadata::Array,
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

    pub(crate) fn extend(
        &mut self,
        buffer: &mut Buffer<'a>,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        if let Some(cached_action) = self.next_action.take() {
            return ControlFlow::Break(cached_action);
        }

        if self.supports_interpolation && self.currently_in_interpolation {
            if buffer.current_byte() == Some(b'}')
                && self.interpolation_started_with_curly_level == current_curly_nest
            {
                // Close interpolation
                let token = Token(
                    TokenValue::tSTRING_DEND,
                    Loc(buffer.pos(), buffer.pos() + 1),
                );
                buffer.skip_byte();
                self.currently_in_interpolation = false;
                return ControlFlow::Break(StringExtendAction::EmitToken { token });
            }

            // yield control to lexer to read interpolated tokens
            return ControlFlow::Break(StringExtendAction::ReadInterpolatedContent);
        }

        let start = buffer.pos();

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
                if buffer.is_eof() {
                    self.handle_eof(buffer, start)?;
                }

                if buffer.const_lookahead(b"#{") {
                    // handle #{ interpolation
                    self.handle_common_interpolation(buffer, start)?;
                }

                if buffer.const_lookahead(b"#@") {
                    // handle #@foo / #@@foo interpolation
                    self.handle_raw_atmark_interpolation(buffer, start)?;
                }

                if buffer.const_lookahead(b"#$") {
                    // handle #$foo interpolation
                    self.handle_raw_gvar_interpolation(buffer, start)?;
                }

                if buffer.lookahead(self.ends_with) {
                    // handle string end
                    self.handle_string_end(buffer, start)?;
                }
                if buffer.const_lookahead(b"\\\n") {
                    // handle explicit line continuation
                    self.handle_string_continuation(buffer, start)?;
                }

                buffer.skip_byte();
            }
        } else {
            loop {
                if buffer.is_eof() {
                    self.handle_eof(buffer, start)?;
                }

                if buffer.lookahead(self.ends_with) {
                    // handle string end
                    self.handle_string_end(buffer, start)?;
                }

                buffer.skip_byte();
            }
        }
    }

    #[must_use]
    fn handle_eof(&self, buffer: &Buffer<'a>, start: usize) -> ControlFlow<StringExtendAction> {
        if let Some(token) = string_content_to_emit(buffer, start, buffer.pos()) {
            ControlFlow::Break(StringExtendAction::EmitToken { token })
        } else {
            ControlFlow::Break(StringExtendAction::EmitEOF)
        }
    }

    #[must_use]
    fn handle_common_interpolation(
        &mut self,
        buffer: &mut Buffer<'a>,
        start: usize,
    ) -> ControlFlow<StringExtendAction> {
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
            ControlFlow::Break(StringExtendAction::EmitToken { token })
        } else {
            ControlFlow::Break(action)
        }
    }

    #[must_use]
    fn handle_raw_atmark_interpolation(
        &mut self,
        buffer: &mut Buffer<'a>,
        start: usize,
    ) -> ControlFlow<StringExtendAction> {
        if let LookaheadAtMarkResult::Ok(token) = lookahead_atmark(buffer, buffer.pos() + 1) {
            // #@foo interpolation
            let interp_action = StringExtendAction::EmitToken {
                token: Token(
                    TokenValue::tSTRING_DVAR,
                    Loc(buffer.pos(), buffer.pos() + 1),
                ),
            };
            let var_action = StringExtendAction::EmitToken { token };
            let string_content = string_content_to_emit(buffer, start, buffer.pos());
            buffer.set_pos(token.loc().end());

            if let Some(token) = string_content {
                self.next_action.add(interp_action);
                self.next_action.add(var_action);
                ControlFlow::Break(StringExtendAction::EmitToken { token })
            } else {
                self.next_action.add(var_action);
                ControlFlow::Break(interp_action)
            }
        } else {
            // just #@ string content without subsequent identifier
            // keep reading
            ControlFlow::Continue(())
        }
    }

    #[must_use]
    fn handle_raw_gvar_interpolation(
        &mut self,
        buffer: &mut Buffer<'a>,
        start: usize,
    ) -> ControlFlow<StringExtendAction> {
        if let LookaheadGvarResult::Ok(token) = lookahead_gvar(buffer, buffer.pos() + 1) {
            // #$foo interpolation
            let interp_action = StringExtendAction::EmitToken {
                token: Token(
                    TokenValue::tSTRING_DVAR,
                    Loc(buffer.pos(), buffer.pos() + 1),
                ),
            };
            let var_action = StringExtendAction::EmitToken { token };
            let string_content = string_content_to_emit(buffer, start, buffer.pos());
            buffer.set_pos(token.loc().end());

            if let Some(token) = string_content {
                self.next_action.add(interp_action);
                self.next_action.add(var_action);
                ControlFlow::Break(StringExtendAction::EmitToken { token })
            } else {
                self.next_action.add(var_action);
                ControlFlow::Break(interp_action)
            }
        } else {
            // just #$ string content without subsequent identifier
            // keep reading
            ControlFlow::Continue(())
        }
    }

    #[must_use]
    fn handle_string_end(
        &mut self,
        buffer: &mut Buffer<'a>,
        start: usize,
    ) -> ControlFlow<StringExtendAction> {
        let string_end_action = StringExtendAction::FoundStringEnd {
            token: Token(
                TokenValue::tSTRING_END,
                Loc(buffer.pos(), buffer.pos() + self.ends_with.len()),
            ),
        };
        let string_content = string_content_to_emit(buffer, start, buffer.pos());
        buffer.set_pos(buffer.pos() + self.ends_with.len());

        if let Some(token) = string_content {
            self.next_action.add(string_end_action);
            ControlFlow::Break(StringExtendAction::EmitToken { token })
        } else {
            ControlFlow::Break(string_end_action)
        }
    }

    #[must_use]
    fn handle_string_continuation(
        &mut self,
        buffer: &mut Buffer<'a>,
        start: usize,
    ) -> ControlFlow<StringExtendAction> {
        // just emit what we've got so far
        // parser will merge two consectuive string literals
        let action = StringExtendAction::EmitToken {
            token: Token(TokenValue::tSTRING_CONTENT, Loc(start, buffer.pos())),
        };
        // and skip escaped NL
        buffer.set_pos(buffer.pos() + 2);
        ControlFlow::Break(action)
    }
}

// Utility helper: checks whether there's recorded string content,
// returns a tSTRING_CONTENT is there's any
fn string_content_to_emit<'a>(buffer: &Buffer<'a>, start: usize, end: usize) -> Option<Token> {
    if start == end {
        None
    } else {
        Some(Token(TokenValue::tSTRING_CONTENT, Loc(start, end)))
    }
}
