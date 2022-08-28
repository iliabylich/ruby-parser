use std::ops::ControlFlow;

use crate::{
    buffer::BufferWithCursor,
    lexer::strings::{
        action::StringExtendAction,
        handlers::{
            handle_eof, handle_escaped_start_or_end, handle_interpolation,
            handle_interpolation_end, handle_string_end, handle_word_separator,
        },
        literal::StringLiteralExtend,
        types::Interpolation,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct WordsInterp {
    interpolation: Interpolation,

    starts_with: u8,
    ends_with: u8,
    ends_with_nesting: usize,
}

impl WordsInterp {
    pub(crate) fn new(starts_with: u8, ends_with: u8, curly_level: usize) -> Self {
        Self {
            interpolation: Interpolation::new(curly_level),
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

impl StringLiteralExtend for WordsInterp {
    fn extend(
        &mut self,
        buffer: &mut BufferWithCursor,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        handle_interpolation_end(buffer, current_curly_nest, &mut self.interpolation)?;

        let start = buffer.pos();

        handle_eof(buffer, start)?;

        loop {
            handle_eof(buffer, start)?;
            handle_word_separator(buffer, start)?;
            handle_escaped_start_or_end(buffer, start, self.starts_with, self.ends_with)?;
            handle_interpolation(&mut self.interpolation, buffer, start)?;

            handle_string_end(
                buffer,
                start,
                self.starts_with,
                self.ends_with,
                &mut self.ends_with_nesting,
            )?;

            buffer.skip_byte()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::lexer::strings::{test_helpers::*, StringLiteral};

    fn literal(starts_with: u8, ends_with: u8) -> StringLiteral {
        StringLiteral::WordsInterp(WordsInterp::new(starts_with, ends_with, 0))
    }

    fn dummy_literal() -> StringLiteral {
        literal(b'[', b']')
    }

    // EOF handling
    assert_emits_eof!(dummy_literal());

    // interpolation END handling
    assert_emits_interpolation_end!(dummy_literal());

    // interpolation VALUE handling
    assert_emits_interpolated_value!(dummy_literal());

    // literal end handling
    assert_emits_string_end!(literal = literal(b'{', b'}'), begin = "{", end = "}");

    assert_does_not_emit_string_content_as_string_end!(
        literal = literal(b'{', b'}'),
        begin = "{",
        end = "}"
    );

    // escape sequences handling
    assert_ignores_escape_sequence!(literal = dummy_literal());

    // escaped literal start/end handling
    assert_emits_escaped_start_or_end!(literal = literal(b'{', b'}'), start = '{', end = '}');

    // line continuation handling
    assert_ignores_line_continuation!(literal = dummy_literal());
}
