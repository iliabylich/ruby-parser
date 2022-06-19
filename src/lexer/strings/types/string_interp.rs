use std::ops::ControlFlow;

use crate::lexer::{
    buffer::BufferWithCursor,
    strings::{
        action::StringExtendAction,
        handlers::{
            handle_eof, handle_escape, handle_interpolation, handle_interpolation_end,
            handle_string_end,
        },
        literal::StringLiteralExtend,
        types::Interpolation,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct StringInterp {
    interpolation: Interpolation,
    ends_with: u8,
}

impl StringInterp {
    pub(crate) fn new(interpolation: Interpolation, ends_with: u8) -> Self {
        Self {
            interpolation,
            ends_with,
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
        handle_interpolation_end(&mut self.interpolation, buffer, current_curly_nest)?;

        let start = buffer.pos();

        loop {
            handle_eof(buffer, start)?;

            handle_escape(buffer, start)?;

            handle_interpolation(&mut self.interpolation, buffer, start)?;
            handle_string_end(self.ends_with, buffer, start)?;

            buffer.skip_byte();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::strings::{test_helpers::*, StringLiteral};

    assert_emits_eof_string_action!(StringLiteral::StringInterp(StringInterp::new(
        Interpolation::new(0),
        b'"'
    )));

    // interpolation END handling
    assert_emits_interpolation_end_action!(StringLiteral::StringInterp(StringInterp::new(
        Interpolation::new(0),
        b'"'
    )));

    // interpolation VALUE handling
    assert_emits_interpolated_value!(StringLiteral::StringInterp(StringInterp::new(
        Interpolation::new(0),
        b'"'
    )));

    assert_emits_string_end!(
        literal = StringLiteral::StringInterp(StringInterp::new(Interpolation::new(0), b'"')),
        input = b"\""
    );

    assert_emits_escaped_slash_u!(
        literal = StringLiteral::StringInterp(StringInterp::new(Interpolation::new(0), b'"'))
    );
}
