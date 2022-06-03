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
    token::{token, Loc, Token},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct Regexp<'a> {
    pub(crate) supports_interpolation: bool,
    pub(crate) currently_in_interpolation: bool,
    pub(crate) ends_with: &'a [u8],
    pub(crate) interpolation_started_with_curly_level: usize,

    pub(crate) next_action: NextAction,
}

generate_default_string_literal_impl!(Regexp);

impl<'a> StringLiteralExtend<'a> for Regexp<'a> {
    fn extend(
        &mut self,
        buffer: &mut Buffer<'a>,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        let mut action = dbg!(self._extend(buffer, current_curly_nest));

        // Regexp has a special handling of string end
        // There can be regexp options after trailing `/`
        //
        // Here we read them and "extend" loc of the tSTRING_END to include options
        match &mut action {
            ControlFlow::Break(StringExtendAction::FoundStringEnd {
                token: Token(_, Loc(_, end)),
            }) if self.ends_with == b"/" => {
                if let Some(regexp_options_end_at) = lookahead_regexp_options(buffer, *end) {
                    *end = regexp_options_end_at;
                    buffer.set_pos(regexp_options_end_at);
                }
            }
            _ => {}
        }

        action
    }
}

impl<'a> Regexp<'a> {
    #[must_use]
    fn _extend(
        &mut self,
        buffer: &mut Buffer<'a>,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        handle_next_action(self)?;
        handle_interpolation_end(self, buffer, current_curly_nest)?;

        let start = buffer.pos();

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
    }
}

fn lookahead_regexp_options(buffer: &mut Buffer, start: usize) -> Option<usize> {
    let mut end = start;
    while matches!(
        buffer.byte_at(end),
        Some(b'o' | b'n' | b'e' | b's' | b'u' | b'i' | b'x' | b'm')
    ) {
        end += 1;
    }
    if start == end {
        None
    } else {
        Some(end)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::strings::test_helpers::*;

    assert_emits_scheduled_string_action!(StringLiteral::regexp());
    assert_emits_eof_string_action!(StringLiteral::regexp());

    // interpolation END handling
    assert_emits_interpolation_end_action!(StringLiteral::regexp()
        .with_ending(b"/")
        .with_interpolation_support(true));

    // interpolation VALUE handling
    assert_emits_interpolated_value!(StringLiteral::regexp()
        .with_ending(b"/")
        .with_interpolation_support(true));

    assert_emits_string_end!(StringLiteral::regexp());

    assert_emits_extend_action!(
        test = test_regexp_options,
        literal = StringLiteral::regexp().with_ending(b"/"),
        input = b"/ox foo",
        action = StringExtendAction::FoundStringEnd {
            token: token!(tSTRING_END, 0, 3)
        },
        pre = |_| {},
        post = |_| {}
    );
}
