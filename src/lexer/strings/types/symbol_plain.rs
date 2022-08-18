use std::ops::ControlFlow;

use crate::{
    buffer::BufferWithCursor,
    lexer::strings::{
        action::StringExtendAction,
        handlers::{handle_eof, handle_string_end},
        literal::StringLiteralExtend,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct SymbolPlain;

impl StringLiteralExtend for SymbolPlain {
    fn extend(
        &mut self,
        buffer: &mut BufferWithCursor,
        _current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        let start = buffer.pos();

        loop {
            handle_eof(buffer, start)?;

            let mut dummy_ends_with_nesting = 0;
            handle_string_end(buffer, start, b'\'', b'\'', &mut dummy_ends_with_nesting)?;

            buffer.skip_byte()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::lexer::strings::{test_helpers::*, StringLiteral};

    fn literal() -> StringLiteral {
        StringLiteral::SymbolPlain(SymbolPlain)
    }

    // EOF handling
    assert_emits_eof!(literal());

    // interpolation END handling
    assert_ignores_interpolation_end!(literal());

    // literal end handling
    assert_emits_string_end!(literal = literal(), begin = "'", end = "'");

    // escape sequences handling
    assert_ignores_escape_sequence!(literal = literal());

    // line continuation handling
    assert_ignores_line_continuation!(literal = literal());
}
