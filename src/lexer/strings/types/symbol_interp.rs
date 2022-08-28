use std::ops::ControlFlow;

use crate::{
    buffer::BufferWithCursor,
    lexer::strings::{
        action::StringExtendAction,
        handlers::{
            handle_eof, handle_escape, handle_interpolation, handle_interpolation_end,
            handle_line_continuation, handle_string_end,
        },
        literal::StringLiteralExtend,
        types::Interpolation,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct SymbolInterp {
    interpolation: Interpolation,
}

impl SymbolInterp {
    pub(crate) fn new(curly_level: usize) -> Self {
        Self {
            interpolation: Interpolation::new(curly_level),
        }
    }

    #[cfg(test)]
    pub(crate) fn enable_interpolation(&mut self) {
        self.interpolation.enabled = true;
    }
}

impl StringLiteralExtend for SymbolInterp {
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

            let mut dummy_ends_with_nesting = 0;
            handle_string_end(buffer, start, b'"', b'"', &mut dummy_ends_with_nesting)?;

            buffer.skip_byte();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::strings::{test_helpers::*, StringLiteral};

    fn literal() -> StringLiteral {
        StringLiteral::SymbolInterp(SymbolInterp::new(0))
    }

    // EOF handling
    assert_emits_eof!(literal());

    // interpolation END handling
    assert_emits_interpolation_end!(literal());

    // interpolation VALUE handling
    assert_emits_interpolated_value!(literal());
    // "#$ "
    // this test is not a part of the generic interpolation testing
    // because WordsInterp lexer treats trailing " " as a word separator
    assert_emits_1_token_and_then_eof!(
        test = test_interp_raw_gvar_no_id,
        literal = literal(),
        input = b"#$ ",
        token = token!(tSTRING_CONTENT, loc!(0, 3)),
        pre = |_| {}
    );

    // literal end handling
    assert_emits_string_end!(literal = literal(), begin = '"', end = '"');

    // escape sequences handling
    assert_emits_escape_sequence!(literal = literal());

    // escaped literal start/end handling
    assert_emits_escaped_start_or_end!(literal = literal(), start = '"', end = '"');

    // line continuation handling
    assert_emits_line_continuation!(literal = literal());
}
