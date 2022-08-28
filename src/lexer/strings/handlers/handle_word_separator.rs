use std::ops::ControlFlow;

use crate::{
    buffer::BufferWithCursor,
    lexer::strings::{action::StringExtendAction, handlers::handle_processed_string_content},
    loc::loc,
    token::token,
};

#[must_use]
pub(crate) fn handle_word_separator(
    buffer: &mut BufferWithCursor,
    start: usize,
) -> std::ops::ControlFlow<StringExtendAction> {
    if buffer.current_byte() == Some(b' ') {
        handle_processed_string_content(buffer.for_lookahead(), start, buffer.pos())?;

        let action = ControlFlow::Break(StringExtendAction::EmitToken {
            token: token!(tSP, loc!(buffer.pos(), buffer.pos() + 1)),
        });
        buffer.skip_byte();
        action
    } else {
        ControlFlow::Continue(())
    }
}
