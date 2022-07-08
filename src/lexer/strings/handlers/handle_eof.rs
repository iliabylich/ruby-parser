use std::ops::ControlFlow;

use crate::lexer::{
    buffer::BufferWithCursor,
    strings::{action::StringExtendAction, handlers::handle_processed_string_content},
};

pub(crate) fn handle_eof(
    buffer: &mut BufferWithCursor,
    start: usize,
) -> ControlFlow<StringExtendAction> {
    if buffer.is_eof() {
        handle_processed_string_content(buffer.for_lookahead(), start, buffer.pos())?;

        ControlFlow::Break(StringExtendAction::EmitEOF { at: buffer.pos() })
    } else {
        ControlFlow::Continue(())
    }
}
