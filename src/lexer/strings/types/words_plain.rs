use std::ops::ControlFlow;

use crate::{
    buffer::BufferWithCursor,
    lexer::strings::{
        action::StringExtendAction,
        handlers::{handle_eof, handle_escaped_start_or_end, handle_string_end},
        literal::StringLiteralExtend,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct WordsPlain {
    starts_with: u8,
    ends_with: u8,
    ends_with_nesting: usize,
}

impl WordsPlain {
    pub(crate) fn new(starts_with: u8, ends_with: u8) -> Self {
        Self {
            starts_with,
            ends_with,
            ends_with_nesting: 0,
        }
    }
}

impl StringLiteralExtend for WordsPlain {
    fn extend(
        &mut self,
        buffer: &mut BufferWithCursor,
        _current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        let start = buffer.pos();

        handle_eof(buffer, start)?;

        todo!("words_plain.extend")
    }
}
