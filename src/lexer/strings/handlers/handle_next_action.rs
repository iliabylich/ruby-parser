use std::ops::ControlFlow;

use crate::lexer::strings::{action::StringExtendAction, types::StringLiteralAttributes};

pub(crate) fn handle_next_action<'a, T>(literal: &mut T) -> ControlFlow<StringExtendAction>
where
    T: StringLiteralAttributes<'a>,
{
    if let Some(cached_action) = literal.next_action_mut().take() {
        return ControlFlow::Break(cached_action);
    }

    ControlFlow::Continue(())
}
