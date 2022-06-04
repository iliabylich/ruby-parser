use std::ops::ControlFlow;

use crate::lexer::{
    buffer::Buffer,
    strings::{action::StringExtendAction, handlers::handle_processed_string_content},
};

pub(crate) fn handle_eof<'a>(
    buffer: &mut Buffer<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction> {
    if buffer.is_eof() {
        handle_processed_string_content(start, buffer.pos())?;

        ControlFlow::Break(StringExtendAction::EmitEOF)
    } else {
        ControlFlow::Continue(())
    }
}
