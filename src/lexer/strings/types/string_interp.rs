use std::{borrow::Cow, ops::ControlFlow};

use crate::{
    lexer::{
        buffer::Buffer,
        strings::{
            action::StringExtendAction,
            handlers::{
                handle_eof, handle_interpolation, handle_interpolation_end, handle_slash_u,
                handle_string_end,
            },
            literal::StringLiteralExtend,
            types::Interpolation,
        },
    },
    token::token,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct StringInterp {
    interpolation: Interpolation,
    ends_with: u8,
}

impl StringInterp {
    pub(crate) fn new(interpolation: Interpolation, ends_with: u8) -> Self {
        Self {
            interpolation,
            ends_with,
        }
    }

    #[cfg(test)]
    pub(crate) fn enable_interpolation(&mut self) {
        self.interpolation.enabled = true;
    }
}

impl<'a> StringLiteralExtend<'a> for StringInterp {
    fn extend(
        &mut self,
        buffer: &mut Buffer<'a>,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction<'a>> {
        handle_interpolation_end(&mut self.interpolation, buffer, current_curly_nest)?;

        let start = buffer.pos();

        loop {
            handle_eof(buffer, start)?;

            // handle_slash_u(buffer, start)?;

            handle_interpolation(&mut self.interpolation, buffer, start)?;
            handle_string_end(self.ends_with, buffer, start)?;

            if buffer.lookahead(b"\\\n") {
                // just emit what we've got so far
                // parser will merge two consectuive string literals
                let end = buffer.pos();
                let action = StringExtendAction::EmitToken {
                    token: token!(
                        tSTRING_CONTENT(Cow::Borrowed(buffer.slice(start, end))),
                        start,
                        end
                    ),
                };
                // and skip escaped NL
                buffer.set_pos(buffer.pos() + 2);
                return ControlFlow::Break(action);
            }

            buffer.skip_byte();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::strings::{test_helpers::*, StringLiteral};

    assert_emits_eof_string_action!(StringLiteral::StringInterp(StringInterp::new(
        Interpolation::new(0),
        b'"'
    )));

    // interpolation END handling
    assert_emits_interpolation_end_action!(StringLiteral::StringInterp(StringInterp::new(
        Interpolation::new(0),
        b'"'
    )));

    // interpolation VALUE handling
    assert_emits_interpolated_value!(StringLiteral::StringInterp(StringInterp::new(
        Interpolation::new(0),
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
                token!(tSTRING_CONTENT(Cow::Borrowed(b"foo")), 1, 4),
                token!(tSTRING_END, 4, 5),
                token!(tEOF, 5, 5)
            ]
        );
    }

    assert_emits_string_end!(
        literal = StringLiteral::StringInterp(StringInterp::new(Interpolation::new(0), b'"')),
        input = b"\""
    );
}
