use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::BufferWithCursor,
        strings::{action::StringExtendAction, literal::StringLiteralExtend, types::Interpolation},
    },
    Loc,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct Heredoc {
    interpolation: Option<Interpolation>,
    id_loc: Loc,

    heredoc_id_ended_at: usize,
    squiggly: bool,
}

impl StringLiteralExtend for Heredoc {
    fn extend(
        &mut self,
        _buffer: &mut BufferWithCursor,
        _current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        todo!("heredoc.extend")
    }
}

impl Heredoc {
    pub(crate) fn new(
        interpolation: Option<Interpolation>,
        id_loc: Loc,
        heredoc_id_ended_at: usize,
        squiggly: bool,
    ) -> Self {
        Self {
            interpolation,
            id_loc,
            heredoc_id_ended_at,
            squiggly,
        }
    }
}
