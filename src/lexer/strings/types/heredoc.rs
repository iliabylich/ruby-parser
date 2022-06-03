use std::ops::ControlFlow;

use crate::lexer::{
    buffer::Buffer,
    strings::{
        action::{NextAction, StringExtendAction},
        literal::StringLiteralExtend,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct Heredoc<'a> {
    supports_interpolation: bool,
    currently_in_interpolation: bool,
    ends_with: &'a [u8],
    interpolation_started_with_curly_level: usize,

    next_action: NextAction,

    heredoc_id_ended_at: usize,
}

impl<'a> StringLiteralExtend<'a> for Heredoc<'a> {
    fn extend(
        &mut self,
        _buffer: &mut Buffer<'a>,
        _current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        todo!()
    }
}
