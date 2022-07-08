use std::ops::ControlFlow;

use crate::lexer::{
    buffer::BufferWithCursor,
    strings::{
        action::StringExtendAction,
        handlers::{handle_eof, handle_escaped_start_or_end, handle_string_end},
        literal::StringLiteralExtend,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct StringPlain {
    starts_with: u8,
    ends_with: u8,
    ends_with_nesting: usize,
}

impl StringPlain {
    pub(crate) fn new(starts_with: u8, ends_with: u8) -> Self {
        Self {
            starts_with,
            ends_with,
            ends_with_nesting: 0,
        }
    }
}

impl StringLiteralExtend for StringPlain {
    fn extend(
        &mut self,
        buffer: &mut BufferWithCursor,
        _current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        let start = buffer.pos();

        loop {
            handle_eof(buffer, start)?;
            handle_escaped_start_or_end(buffer, start, self.starts_with, self.ends_with)?;
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
        StringLiteral::StringPlain(StringPlain::new(starts_with, ends_with))
    }

    fn dummy_literal() -> StringLiteral {
        literal(b'\'', b'\'')
    }

    // EOF handling
    assert_emits_eof!(dummy_literal());

    // interpolation END handling
    assert_ignores_interpolation_end!(dummy_literal());

    // literal end handling
    assert_emits_string_end!(literal = literal(b'{', b'}'), begin = "{", end = "}");

    // escape sequences handling
    assert_ignores_escape_sequence!(literal = dummy_literal());

    // escaped literal start/end handling
    assert_emits_escaped_start_or_end!(literal = literal(b'{', b'}'), start = "{", end = "}");

    // line continuation handling
    assert_ignores_line_continuation!(literal = dummy_literal());
}
