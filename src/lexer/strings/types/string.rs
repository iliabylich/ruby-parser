use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::Buffer,
        strings::{
            action::{NextAction, StringExtendAction},
            handlers::{
                contracts::{HasInterpolation, HasNextAction},
                handle_eof, handle_interpolation, handle_interpolation_end, handle_next_action,
                handle_string_end,
            },
            literal::StringLiteralExtend,
        },
    },
    token::token,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct String {
    supports_interpolation: bool,
    currently_in_interpolation: bool,
    ends_with: u8,
    interpolation_started_with_curly_level: usize,

    next_action: NextAction,
}

impl String {
    pub(crate) fn new(interp: bool, ends_with: u8, curly_level: usize) -> Self {
        Self {
            supports_interpolation: interp,
            currently_in_interpolation: false,
            ends_with,
            interpolation_started_with_curly_level: curly_level,
            next_action: NextAction::NoAction,
        }
    }
}

impl HasNextAction for String {
    fn next_action_mut(&mut self) -> &mut NextAction {
        &mut self.next_action
    }
}

impl HasInterpolation for String {
    fn currently_in_interpolation(&self) -> bool {
        self.currently_in_interpolation
    }

    fn currently_in_interpolation_mut(&mut self) -> &mut bool {
        &mut self.currently_in_interpolation
    }

    fn supports_interpolation(&self) -> bool {
        self.supports_interpolation
    }

    fn interpolation_started_with_curly_level(&self) -> usize {
        self.interpolation_started_with_curly_level
    }
}

impl<'a> StringLiteralExtend<'a> for String {
    fn extend(
        &mut self,
        buffer: &mut Buffer<'a>,
        current_curly_nest: usize,
    ) -> ControlFlow<crate::lexer::strings::action::StringExtendAction> {
        // debug_assert!(!self.ends_with.is_empty());

        handle_next_action(self)?;
        handle_interpolation_end(self, buffer, current_curly_nest)?;

        let start = buffer.pos();

        if self.supports_interpolation {
            loop {
                handle_eof(buffer, start)?;
                handle_interpolation(self, buffer, start)?;
                handle_string_end(self, self.ends_with, buffer, start)?;

                if buffer.lookahead(b"\\\n") {
                    // just emit what we've got so far
                    // parser will merge two consectuive string literals
                    let action = StringExtendAction::EmitToken {
                        token: token!(tSTRING_CONTENT, start, buffer.pos()),
                    };
                    // and skip escaped NL
                    buffer.set_pos(buffer.pos() + 2);
                    return ControlFlow::Break(action);
                }

                buffer.skip_byte();
            }
        } else {
            loop {
                handle_eof(buffer, start)?;
                handle_string_end(self, self.ends_with, buffer, start)?;
                buffer.skip_byte()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::strings::{test_helpers::*, types::String, StringLiteral};

    assert_emits_scheduled_string_action!(StringLiteral::String(String::new(true, b'"', 0)));
    assert_emits_eof_string_action!(StringLiteral::String(String::new(true, b'"', 0)));

    // interpolation END handling
    assert_emits_interpolation_end_action!(StringLiteral::String(String::new(true, b'"', 0)));
    assert_emits_token!(
        test = test_rcurly_with_no_interp_support,
        literal = StringLiteral::String(String::new(false, b'\'', 0)),
        input = b"}",
        token = token!(tSTRING_CONTENT, 0, 1),
        pre = |_| {},
        post = |_| {}
    );

    // interpolation VALUE handling
    assert_emits_interpolated_value!(StringLiteral::String(String::new(true, b'"', 0)));

    #[test]
    fn test_string_plain_non_interp() {
        use crate::{lexer::Lexer, token::token};
        let mut lexer = Lexer::new(b"'foo'");
        assert_eq!(
            lexer.tokenize_until_eof(),
            vec![
                token!(tSTRING_BEG, 0, 1),
                token!(tSTRING_CONTENT, 1, 4),
                token!(tSTRING_END, 4, 5),
                token!(tEOF, 5, 5)
            ]
        );
    }

    assert_emits_string_end!(
        literal = StringLiteral::String(String::new(true, b'"', 0)),
        input = b"\""
    );
}
