use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::Buffer,
        strings::{
            action::{NextAction, StringExtendAction},
            handlers::{
                handle_eof, handle_interpolation, handle_interpolation_end, handle_next_action,
                handle_string_end,
            },
            literal::StringLiteralExtend,
            types::{HasInterpolation, HasNextAction, Interpolation},
        },
    },
    token::token,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct String {
    interpolation: Interpolation,
    ends_with: u8,

    next_action: NextAction,
}

impl String {
    pub(crate) fn new(interpolation: Interpolation, ends_with: u8) -> Self {
        Self {
            interpolation,
            ends_with,

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
    fn interpolation(&self) -> &Interpolation {
        &self.interpolation
    }

    fn interpolation_mut(&mut self) -> &mut Interpolation {
        &mut self.interpolation
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

        match self.interpolation {
            Interpolation::Available { .. } => loop {
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
            },
            Interpolation::Disabled => loop {
                handle_eof(buffer, start)?;
                handle_string_end(self, self.ends_with, buffer, start)?;
                buffer.skip_byte()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::strings::{test_helpers::*, StringLiteral};

    assert_emits_scheduled_string_action!(StringLiteral::String(String::new(
        Interpolation::available(0),
        b'"'
    )));
    assert_emits_eof_string_action!(StringLiteral::String(String::new(
        Interpolation::available(0),
        b'"'
    )));

    // interpolation END handling
    assert_emits_interpolation_end_action!(StringLiteral::String(String::new(
        Interpolation::available(0),
        b'"'
    )));
    assert_emits_token!(
        test = test_rcurly_with_no_interp_support,
        literal = StringLiteral::String(String::new(Interpolation::Disabled, b'\'')),
        input = b"}",
        token = token!(tSTRING_CONTENT, 0, 1),
        pre = |_| {},
        post = |_| {}
    );

    // interpolation VALUE handling
    assert_emits_interpolated_value!(StringLiteral::String(String::new(
        Interpolation::available(0),
        b'"'
    )));

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
        literal = StringLiteral::String(String::new(Interpolation::available(0), b'"')),
        input = b"\""
    );
}
