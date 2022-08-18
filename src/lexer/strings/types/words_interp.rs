use std::ops::ControlFlow;

use crate::{
    buffer::BufferWithCursor,
    lexer::strings::{
        action::StringExtendAction, handlers::handle_eof, literal::StringLiteralExtend,
        types::Interpolation,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct WordsInterp {
    interolation: Interpolation,

    starts_with: u8,
    ends_with: u8,
    ends_with_nesting: usize,
}

impl WordsInterp {
    pub(crate) fn new(starts_with: u8, ends_with: u8, curly_level: usize) -> Self {
        Self {
            interolation: Interpolation::new(curly_level),
            starts_with,
            ends_with,
            ends_with_nesting: 0,
        }
    }
}

impl StringLiteralExtend for WordsInterp {
    fn extend(
        &mut self,
        buffer: &mut BufferWithCursor,
        _current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        let start = buffer.pos();

        handle_eof(buffer, start)?;

        todo!("words_interp.extend")
    }
}
