use std::ops::ControlFlow;

use crate::lexer::{
    buffer::Buffer,
    strings::{action::StringExtendAction, handlers::handle_eof, literal::StringLiteralExtend},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct Symbol {
    supports_interpolation: bool,
    currently_in_interpolation: bool,
    interpolation_started_with_curly_level: usize,
}

impl Symbol {
    pub(crate) fn new(interp: bool, curly_level: usize) -> Self {
        Self {
            supports_interpolation: interp,
            currently_in_interpolation: false,
            interpolation_started_with_curly_level: curly_level,
        }
    }
}

impl<'a> StringLiteralExtend<'a> for Symbol {
    fn extend(
        &mut self,
        buffer: &mut Buffer<'a>,
        _current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction<'a>> {
        let start = buffer.pos();

        handle_eof(buffer, start)?;

        todo!("implement symbol.extend")
    }
}
