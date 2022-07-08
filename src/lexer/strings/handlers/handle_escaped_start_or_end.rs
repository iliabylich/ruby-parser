use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::BufferWithCursor,
        strings::{action::StringExtendAction, handlers::handle_processed_string_content},
    },
    loc::loc,
    token::token,
};

pub(crate) fn handle_escaped_start_or_end(
    buffer: &mut BufferWithCursor,
    start: usize,
    starts_with: u8,
    ends_with: u8,
) -> ControlFlow<StringExtendAction> {
    if buffer.current_byte() == Some(b'\\') {
        match buffer.byte_at(buffer.pos() + 1) {
            Some(byte) if byte == starts_with || byte == ends_with => {
                handle_processed_string_content(buffer.for_lookahead(), start, buffer.pos())?;

                let action = ControlFlow::Break(StringExtendAction::EmitToken {
                    token: token!(tSTRING_CONTENT, loc!(buffer.pos(), buffer.pos() + 2), byte),
                });
                buffer.set_pos(buffer.pos() + 2);
                return action;
            }
            _ => {}
        }
    }

    ControlFlow::Continue(())
}
