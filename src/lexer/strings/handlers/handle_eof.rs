use std::ops::ControlFlow;

use crate::lexer::{
    buffer::Buffer,
    strings::{action::StringExtendAction, handlers::string_content_to_emit},
};

pub(crate) fn handle_eof<'a>(
    buffer: &mut Buffer<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction> {
    if buffer.is_eof() {
        if let Some(token) = string_content_to_emit(start, buffer.pos()) {
            ControlFlow::Break(StringExtendAction::EmitToken { token })
        } else {
            ControlFlow::Break(StringExtendAction::EmitEOF)
        }
    } else {
        ControlFlow::Continue(())
    }
}
