use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::BufferWithCursor,
        strings::{action::StringExtendAction, handlers::handle_processed_string_content},
    },
    loc::loc,
    string_content::StringContent,
    token::token,
};

pub(crate) fn handle_line_continuation<'a>(
    buffer: &mut BufferWithCursor<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction<'a>> {
    if buffer.lookahead(b"\\\n") {
        handle_processed_string_content(buffer.for_lookahead(), start, buffer.pos())?;

        let action = ControlFlow::Break(StringExtendAction::EmitToken {
            token: token!(
                tSTRING_CONTENT(StringContent::Borrowed(b"")),
                loc!(buffer.pos(), buffer.pos() + 2)
            ),
        });
        buffer.set_pos(buffer.pos() + 2);
        action
    } else {
        ControlFlow::Continue(())
    }
}
