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
            types::generate_default_string_literal_impl,
        },
    },
    token::token,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct String<'a> {
    pub(crate) supports_interpolation: bool,
    pub(crate) currently_in_interpolation: bool,
    pub(crate) ends_with: &'a [u8],
    pub(crate) interpolation_started_with_curly_level: usize,

    pub(crate) next_action: NextAction,
}

generate_default_string_literal_impl!(String);

impl<'a> StringLiteralExtend<'a> for String<'a> {
    fn extend(
        &mut self,
        buffer: &mut Buffer<'a>,
        current_curly_nest: usize,
    ) -> ControlFlow<crate::lexer::strings::action::StringExtendAction> {
        handle_next_action(self)?;
        handle_interpolation_end(self, buffer, current_curly_nest)?;

        let start = buffer.pos();

        if self.supports_interpolation {
            loop {
                handle_eof(buffer, start)?;
                handle_interpolation(self, buffer, start)?;
                handle_string_end(self, buffer, start)?;

                if buffer.const_lookahead(b"\\\n") {
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
                handle_string_end(self, buffer, start)?;
                buffer.skip_byte()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::strings::test_helpers::*;

    assert_emits_scheduled_string_action!(StringLiteral::string());
    assert_emits_eof_string_action!(StringLiteral::string());

    // interpolation END handling
    assert_emits_interpolation_end_action!(StringLiteral::string()
        .with_ending(b"\"")
        .with_interpolation_support(true));
    assert_emits_token!(
        test = test_rcurly_with_no_interp_support,
        literal = StringLiteral::string()
            .with_ending(b"'")
            .with_interpolation_support(false),
        input = b"}",
        token = token!(tSTRING_CONTENT, 0, 1),
        pre = |_| {},
        post = |_| {}
    );

    // interpolation VALUE handling
    assert_emits_interpolated_value!(StringLiteral::string()
        .with_ending(b"\"")
        .with_interpolation_support(true));

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

    assert_emits_string_end!(StringLiteral::string());
}
