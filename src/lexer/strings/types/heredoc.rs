use std::ops::ControlFlow;

use crate::lexer::{
    buffer::BufferWithCursor,
    strings::{action::StringExtendAction, literal::StringLiteralExtend, types::Interpolation},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct Heredoc<'a> {
    interpolation: Option<Interpolation>,
    ends_with: &'a [u8],

    heredoc_id_ended_at: usize,
    squiggly: bool,
}

impl<'a> StringLiteralExtend<'a> for Heredoc<'a> {
    fn extend(
        &mut self,
        _buffer: &mut BufferWithCursor<'a>,
        _current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction<'a>> {
        todo!()
    }
}

impl<'a> Heredoc<'a> {
    pub(crate) fn new(
        interpolation: Option<Interpolation>,
        ends_with: &'a [u8],
        heredoc_id_ended_at: usize,
        squiggly: bool,
    ) -> Self {
        Self {
            interpolation,
            ends_with,
            heredoc_id_ended_at,
            squiggly,
        }
    }
}
