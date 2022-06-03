use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::{Buffer, Pattern},
        strings::{
            action::StringExtendAction, handlers::contracts::HasNextAction,
            handlers::string_content_to_emit,
        },
    },
    token::token,
};

pub(crate) fn handle_string_end<'a, L, P>(
    literal: &mut L,
    ends_with: P,
    buffer: &mut Buffer<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction>
where
    L: HasNextAction,
    P: Pattern,
{
    if buffer.lookahead(&ends_with) {
        let string_end_action = StringExtendAction::FoundStringEnd {
            token: token!(tSTRING_END, buffer.pos(), buffer.pos() + ends_with.length()),
        };
        let string_content = string_content_to_emit(start, buffer.pos());
        buffer.set_pos(buffer.pos() + ends_with.length());

        if let Some(token) = string_content {
            literal.next_action_mut().add(string_end_action);
            ControlFlow::Break(StringExtendAction::EmitToken { token })
        } else {
            ControlFlow::Break(string_end_action)
        }
    } else {
        ControlFlow::Continue(())
    }
}
