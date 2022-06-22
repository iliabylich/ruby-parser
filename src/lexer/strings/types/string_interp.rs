use std::ops::ControlFlow;

use crate::lexer::{
    buffer::BufferWithCursor,
    strings::{
        action::StringExtendAction,
        handlers::{
            handle_eof, handle_escape, handle_interpolation, handle_interpolation_end,
            handle_line_continuation, handle_string_end,
        },
        literal::StringLiteralExtend,
        types::Interpolation,
    },
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct StringInterp {
    interpolation: Interpolation,

    starts_with: u8,
    ends_with: u8,
    ends_with_nesting: usize,
}

impl StringInterp {
    pub(crate) fn new(interpolation: Interpolation, starts_with: u8, ends_with: u8) -> Self {
        Self {
            interpolation,
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

impl<'a> StringLiteralExtend<'a> for StringInterp {
    fn extend(
        &mut self,
        buffer: &mut BufferWithCursor<'a>,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction<'a>> {
        handle_interpolation_end(buffer, current_curly_nest, &mut self.interpolation)?;

        let start = buffer.pos();

        loop {
            handle_eof(buffer, start)?;

            handle_line_continuation(buffer, start)?;

            handle_escape(buffer, start)?;

            handle_interpolation(&mut self.interpolation, buffer, start)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::strings::{test_helpers::*, StringLiteral};

    fn literal(starts_with: u8, ends_with: u8) -> StringLiteral<'static> {
        StringLiteral::StringInterp(StringInterp::new(
            Interpolation::new(0),
            starts_with,
            ends_with,
        ))
    }

    fn dummy_literal() -> StringLiteral<'static> {
        literal(b'"', b'"')
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
    assert_emits_escaped_start_or_end!(literal = literal(b'{', b'}'), start = "{", end = "}");

    // line continuation handling
    assert_emits_line_continuation!(literal = dummy_literal());
}
