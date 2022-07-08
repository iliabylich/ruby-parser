use std::ops::ControlFlow;

use crate::{
    lexer::{buffer::Buffer, strings::action::StringExtendAction},
    loc::loc,
    token::token,
};

#[must_use]
pub(crate) fn handle_processed_string_content(
    _buffer: &Buffer,
    start: usize,
    end: usize,
) -> std::ops::ControlFlow<StringExtendAction> {
    if start == end {
        ControlFlow::Continue(())
    } else {
        ControlFlow::Break(StringExtendAction::EmitToken {
            token: token!(tSTRING_CONTENT, loc!(start, end)),
        })
    }
}
