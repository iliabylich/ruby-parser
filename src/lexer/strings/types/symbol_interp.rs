use std::ops::ControlFlow;

use crate::{
    buffer::BufferWithCursor,
    lexer::strings::{
        action::StringExtendAction, handlers::handle_eof, literal::StringLiteralExtend,
        types::Interpolation,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct SymbolInterp {
    interpolation: Interpolation,
}

impl SymbolInterp {
    pub(crate) fn new(curly_level: usize) -> Self {
        Self {
            interpolation: Interpolation::new(curly_level),
        }
    }
}

impl StringLiteralExtend for SymbolInterp {
    fn extend(
        &mut self,
        buffer: &mut BufferWithCursor,
        _current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        let start = buffer.pos();

        handle_eof(buffer, start)?;

        todo!("symbol_interp.extend")
    }
}
