use std::ops::ControlFlow;

use crate::{
    buffer::BufferWithCursor,
    lexer::strings::{action::StringExtendAction, handlers::handle_processed_string_content},
    loc::loc,
    token::token,
};

pub(crate) fn handle_line_continuation(
    buffer: &mut BufferWithCursor,
    start: usize,
) -> ControlFlow<StringExtendAction> {
    if buffer.lookahead(b"\\\n") {
        handle_processed_string_content(buffer.for_lookahead(), start, buffer.pos())?;

        let action = ControlFlow::Break(StringExtendAction::EmitToken {
            token: token!(tSTRING_CONTENT, loc!(buffer.pos(), buffer.pos() + 2)),
        });
        buffer.set_pos(buffer.pos() + 2);
        action
    } else {
        ControlFlow::Continue(())
    }
}
