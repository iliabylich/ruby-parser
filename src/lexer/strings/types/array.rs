use std::ops::ControlFlow;

use crate::lexer::{
    buffer::Buffer,
    strings::{
        action::{NextAction, StringExtendAction},
        handlers::handle_eof,
        literal::StringLiteralExtend,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct Array {
    supports_interpolation: bool,
    currently_in_interpolation: bool,
    ends_with: u8,
    interpolation_started_with_curly_level: usize,

    next_action: NextAction,
}

impl Array {
    pub(crate) fn new(interp: bool, ends_with: u8, curly_level: usize) -> Self {
        Self {
            supports_interpolation: interp,
            currently_in_interpolation: false,
            ends_with,
            interpolation_started_with_curly_level: curly_level,
            next_action: NextAction::NoAction,
        }
    }
}

impl<'a> StringLiteralExtend<'a> for Array {
    fn extend(
        &mut self,
        buffer: &mut Buffer<'a>,
        _current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        let start = buffer.pos();

        handle_eof(buffer, start)?;
        todo!("implement word_array.extend")
    }
}
