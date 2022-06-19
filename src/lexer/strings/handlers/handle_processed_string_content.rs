use std::ops::ControlFlow;

use crate::{
    lexer::{buffer::Buffer, string_content::StringContent, strings::action::StringExtendAction},
    token::token,
};

#[must_use]
pub(crate) fn handle_processed_string_content<'a>(
    buffer: &Buffer<'a>,
    start: usize,
    end: usize,
) -> std::ops::ControlFlow<StringExtendAction<'a>> {
    if start == end {
        ControlFlow::Continue(())
    } else {
        ControlFlow::Break(StringExtendAction::EmitToken {
            token: token!(
                tSTRING_CONTENT(StringContent::from(buffer.slice(start, end).expect("bug"))),
                start,
                end
            ),
        })
    }
}