use std::ops::ControlFlow;

use crate::lexer::strings::{action::StringExtendAction, types::HasNextAction};

pub(crate) fn handle_next_action<'a, T>(literal: &mut T) -> ControlFlow<StringExtendAction>
where
    T: HasNextAction,
{
    if let Some(cached_action) = literal.next_action_mut().take() {
        return ControlFlow::Break(cached_action);
    }

    ControlFlow::Continue(())
}
