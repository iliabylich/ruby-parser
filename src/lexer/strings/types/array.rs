use std::ops::ControlFlow;

use crate::{
    buffer::BufferWithCursor,
    lexer::strings::{
        action::StringExtendAction, handlers::handle_eof, literal::StringLiteralExtend,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct Array {
    interpolated: bool,
    currently_in_interpolation: bool,

    starts_with: u8,
    ends_with: u8,

    interpolation_started_with_curly_level: usize,
}

impl Array {
    pub(crate) fn new(
        interpolated: bool,
        starts_with: u8,
        ends_with: u8,
        curly_level: usize,
    ) -> Self {
        Self {
            interpolated: interpolated,
            currently_in_interpolation: false,
            starts_with,
            ends_with,
            interpolation_started_with_curly_level: curly_level,
        }
    }
}

impl StringLiteralExtend for Array {
    fn extend(
        &mut self,
        buffer: &mut BufferWithCursor,
        _current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        let start = buffer.pos();

        handle_eof(buffer, start)?;
        todo!("implement word_array.extend")
    }
}
