use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::Buffer,
        strings::{
            action::StringExtendAction,
            handlers::{
                handle_eof, handle_interpolation, handle_interpolation_end, handle_string_end,
            },
            literal::StringLiteralExtend,
            types::{HasInterpolation, Interpolation},
        },
    },
    token::{token, Loc, Token},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Regexp {
    interpolation: Interpolation,
    ends_with: u8,
}

impl Regexp {
    pub(crate) fn new(ends_with: u8, curly_level: usize) -> Self {
        Self {
            interpolation: Interpolation {
                enabled: false,
                curly_nest: curly_level,
            },
            ends_with,
        }
    }
}

impl HasInterpolation for Regexp {
    fn interpolation(&self) -> &Interpolation {
        &self.interpolation
    }

    fn interpolation_mut(&mut self) -> &mut Interpolation {
        &mut self.interpolation
    }
}

impl<'a> StringLiteralExtend<'a> for Regexp {
    fn extend(
        &mut self,
        buffer: &mut Buffer<'a>,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        let mut action = self._extend(buffer, current_curly_nest);

        // Regexp has a special handling of string end
        // There can be regexp options after trailing `/`
        //
        // Here we read them and "extend" loc of the tSTRING_END to include options
        match &mut action {
            ControlFlow::Break(StringExtendAction::FoundStringEnd {
                token: Token(_, Loc(_, end)),
            }) if self.ends_with == b'/' => {
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

impl Regexp {
    #[must_use]
    fn _extend(
        &mut self,
        buffer: &mut Buffer,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        handle_interpolation_end(self, buffer, current_curly_nest)?;

        let start = buffer.pos();

        loop {
            handle_eof(buffer, start)?;
            handle_interpolation(self, buffer, start)?;
            handle_string_end(self.ends_with, buffer, start)?;

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
    use crate::lexer::strings::{test_helpers::*, types::Regexp, StringLiteral};

    assert_emits_eof_string_action!(StringLiteral::Regexp(Regexp::new(b'/', 0)));

    // interpolation END handling
    assert_emits_interpolation_end_action!(StringLiteral::Regexp(Regexp::new(b'/', 0)));

    // interpolation VALUE handling
    assert_emits_interpolated_value!(StringLiteral::Regexp(Regexp::new(b'/', 0)));

    assert_emits_string_end!(
        literal = StringLiteral::Regexp(Regexp::new(b'/', 0)),
        input = b"/"
    );

    assert_emits_extend_action!(
        test = test_regexp_options,
        literal = StringLiteral::Regexp(Regexp::new(b'/', 0)),
        input = b"/ox foo",
        action = StringExtendAction::FoundStringEnd {
            token: token!(tSTRING_END, 0, 3)
        },
        pre = |_| {},
        post = |_| {}
    );
}
