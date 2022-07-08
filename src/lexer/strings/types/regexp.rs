use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::{Buffer, BufferWithCursor, Lookahead},
        strings::{
            action::StringExtendAction,
            handlers::{
                handle_eof, handle_escape, handle_interpolation, handle_interpolation_end,
                handle_line_continuation, handle_processed_string_content, handle_string_end,
            },
            literal::StringLiteralExtend,
            types::Interpolation,
        },
    },
    loc::loc,
    token::token,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Regexp {
    interpolation: Interpolation,

    starts_with: u8,
    ends_with: u8,
    ends_with_nesting: usize,
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
            ends_with_nesting: 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn enable_interpolation(&mut self) {
        self.interpolation.enabled = true;
    }
}

impl StringLiteralExtend for Regexp {
    fn extend(
        &mut self,
        buffer: &mut BufferWithCursor,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        handle_interpolation_end(buffer, current_curly_nest, &mut self.interpolation)?;

        let start = buffer.pos();

        loop {
            handle_eof(buffer, start)?;

            handle_line_continuation(buffer, start)?;

            handle_escape(buffer, start)?;

            handle_interpolation(&mut self.interpolation, buffer, start)?;

            // first check if there's a '/oix' regexp end
            handle_regexp_end_with_options(buffer, start, self.ends_with)?;
            // and then check a normal regexp end
            handle_string_end(
                buffer,
                start,
                self.starts_with,
                self.ends_with,
                &mut self.ends_with_nesting,
            )?;

            buffer.skip_byte();
        }
    }
}

struct RegexpOptions {
    length: usize,
}

impl Lookahead for RegexpOptions {
    type Output = Option<Self>;

    fn lookahead(buffer: &Buffer, start: usize) -> Self::Output {
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

fn handle_regexp_end_with_options(
    buffer: &mut BufferWithCursor,
    start: usize,
    ends_with: u8,
) -> ControlFlow<StringExtendAction> {
    if ends_with == b'/' && buffer.current_byte() == Some(b'/') {
        // definitely a /foo/ regexp end
        handle_processed_string_content(buffer.for_lookahead(), start, buffer.pos())?;

        if let Some(RegexpOptions { length }) =
            RegexpOptions::lookahead(buffer.for_lookahead(), buffer.pos() + 1)
        {
            let token_end = buffer.pos() + 1 + length;
            let action = ControlFlow::Break(StringExtendAction::FoundStringEnd {
                token: token!(tSTRING_END, loc!(buffer.pos(), token_end)),
            });
            buffer.set_pos(token_end);
            return action;
        }
    }

    ControlFlow::Continue(())
}

#[cfg(test)]
mod tests {
    use crate::lexer::strings::{test_helpers::*, types::Regexp, StringLiteral};

    fn literal(starts_with: u8, ends_with: u8) -> StringLiteral {
        StringLiteral::Regexp(Regexp::new(starts_with, ends_with, 0))
    }

    fn dummy_literal() -> StringLiteral {
        literal(b'/', b'/')
    }

    // EOF handling
    assert_emits_eof!(dummy_literal());

    // interpolation END handling
    assert_emits_interpolation_end!(dummy_literal());

    // interpolation VALUE handling
    assert_emits_interpolated_value!(dummy_literal());

    // literal end handling
    assert_emits_string_end!(literal = literal(b'{', b'}'), begin = '{', end = '}');

    // escape sequences handling
    assert_emits_escape_sequence!(literal = dummy_literal());

    // escaped literal start/end handling
    assert_emits_escaped_start_or_end!(literal = literal(b'{', b'}'), start = '{', end = '}');

    // line continuation handling
    assert_emits_line_continuation!(literal = dummy_literal());

    // regexp options handling
    assert_emits_extend_action!(
        test = test_regexp_options,
        literal = literal(b'/', b'/'),
        input = b"/ox",
        action = StringExtendAction::FoundStringEnd {
            token: token!(tSTRING_END, loc!(0, 3))
        },
        pre = |_| {},
        post = |action: StringExtendAction| {
            assert_eq!(
                action,
                StringExtendAction::EmitEOF { at: 3 },
                "expected EOF after tSTRING_END"
            )
        }
    );
    assert_emits_extend_action!(
        test = test_regexp_options_for_percent_r_regexp,
        literal = literal(b'{', b'}'),
        input = b"}ox",
        action = StringExtendAction::FoundStringEnd {
            token: token!(tSTRING_END, loc!(0, 1))
        },
        pre = |_| {},
        post = |_| {}
    );
}
