use std::ops::ControlFlow;

use crate::{lexer::strings::action::StringExtendAction, token::token};

#[must_use]
pub(crate) fn handle_processed_string_content(
    start: usize,
    end: usize,
) -> std::ops::ControlFlow<StringExtendAction> {
    if start == end {
        ControlFlow::Continue(())
    } else {
        ControlFlow::Break(StringExtendAction::EmitToken {
            token: token!(tSTRING_CONTENT, start, end),
        })
    }
}
