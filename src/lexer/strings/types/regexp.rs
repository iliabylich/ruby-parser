use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::{Buffer, BufferWithCursor, Lookahead},
        string_content::StringContent,
        strings::{
            action::StringExtendAction,
            handlers::{
                handle_eof, handle_interpolation, handle_interpolation_end, handle_string_end,
            },
            literal::StringLiteralExtend,
            types::Interpolation,
        },
    },
    token::{token, Loc, Token},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Regexp {
    interpolation: Interpolation,

    starts_with: u8,
    ends_with: u8,
}

impl Regexp {
    pub(crate) fn new(starts_with: u8, ends_with: u8, curly_level: usize) -> Self {
        Self {
            interpolation: Interpolation {
                enabled: false,
                curly_nest: curly_level,
            },
            starts_with,
            ends_with,
        }
    }

    #[cfg(test)]
    pub(crate) fn enable_interpolation(&mut self) {
        self.interpolation.enabled = true;
    }
}

impl<'a> StringLiteralExtend<'a> for Regexp {
    fn extend(
        &mut self,
        buffer: &mut BufferWithCursor<'a>,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction<'a>> {
        let mut action = self._extend(buffer, current_curly_nest);

        // Regexp has a special handling of string end
        // There can be regexp options after trailing `/`
        //
        // Here we read them and "extend" loc of the tSTRING_END to include options
        match &mut action {
            ControlFlow::Break(StringExtendAction::FoundStringEnd {
                token: Token(_, Loc(_, end)),
            }) if self.ends_with == b'/' => {
                if let Some(RegexpOptions { length }) =
                    RegexpOptions::lookahead(buffer.for_lookahead(), *end)
                {
                    *end += length;
                    buffer.set_pos(*end);
                }
            }
            _ => {}
        }

        action
    }
}

impl Regexp {
    #[must_use]
    fn _extend<'a>(
        &mut self,
        buffer: &mut BufferWithCursor<'a>,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction<'a>> {
        handle_interpolation_end(&mut self.interpolation, buffer, current_curly_nest)?;

        let start = buffer.pos();

        loop {
            handle_eof(buffer, start)?;
            handle_interpolation(&mut self.interpolation, buffer, start)?;
            handle_string_end(self.ends_with, buffer, start)?;

            if buffer.lookahead(b"\\\n") {
                // just emit what we've got so far
                // parser will merge two consectuive string literals
                let end = buffer.pos();
                let action = StringExtendAction::EmitToken {
                    token: token!(
                        tSTRING_CONTENT(StringContent::from(
                            buffer.slice(start, end).expect("bug")
                        )),
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

struct RegexpOptions {
    length: usize,
}

impl<'a> Lookahead<'a> for RegexpOptions {
    type Output = Option<Self>;

    fn lookahead(buffer: &Buffer<'a>, start: usize) -> Self::Output {
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
            Some(RegexpOptions {
                length: end - start,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::strings::{test_helpers::*, types::Regexp, StringLiteral};

    assert_emits_eof!(StringLiteral::Regexp(Regexp::new(b'/', b'/', 0)));

    // interpolation END handling
    assert_emits_interpolation_end!(StringLiteral::Regexp(Regexp::new(b'/', b'/', 0)));

    // interpolation VALUE handling
    assert_emits_interpolated_value!(StringLiteral::Regexp(Regexp::new(b'/', b'/', 0)));

    assert_emits_string_end!(
        literal = StringLiteral::Regexp(Regexp::new(b'/', b'/', 0)),
        input = b"/"
    );

    assert_emits_extend_action!(
        test = test_regexp_options,
        literal = StringLiteral::Regexp(Regexp::new(b'/', b'/', 0)),
        input = b"/ox",
        action = StringExtendAction::FoundStringEnd {
            token: token!(tSTRING_END, 0, 3)
        },
        pre = |_| {},
        post = |action: StringExtendAction| {
            assert_eq!(
                action,
                StringExtendAction::EmitEOF { at: 3 },
                "2nd action daction doesn't match"
            )
        }
    );
}
