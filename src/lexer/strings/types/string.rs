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
