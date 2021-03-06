use std::ops::ControlFlow;

use crate::{
    buffer::BufferWithCursor,
    lexer::strings::{
        action::StringExtendAction, handlers::handle_eof, literal::StringLiteralExtend,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct Symbol {
    interpolated: bool,
    currently_in_interpolation: bool,
    interpolation_started_with_curly_level: usize,
}

impl Symbol {
    pub(crate) fn new(interpolated: bool, curly_level: usize) -> Self {
        Self {
            interpolated,
            currently_in_interpolation: false,
            interpolation_started_with_curly_level: curly_level,
        }
    }
}

impl StringLiteralExtend for Symbol {
    fn extend(
        &mut self,
        buffer: &mut BufferWithCursor,
        _current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        let start = buffer.pos();

        handle_eof(buffer, start)?;

        todo!("symbol.extend")
    }
}
