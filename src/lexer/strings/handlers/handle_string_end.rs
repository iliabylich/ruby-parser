use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::Buffer,
        strings::{
            action::StringExtendAction, handlers::string_content_to_emit,
            types::StringLiteralAttributes,
        },
    },
    token::token,
};

pub(crate) fn handle_string_end<'a, T>(
    literal: &mut T,
    buffer: &mut Buffer<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction>
where
    T: StringLiteralAttributes<'a>,
{
    if buffer.lookahead(literal.ends_with()) {
        let string_end_action = StringExtendAction::FoundStringEnd {
            token: token!(
                tSTRING_END,
                buffer.pos(),
                buffer.pos() + literal.ends_with().len()
            ),
        };
        let string_content = string_content_to_emit(start, buffer.pos());
        buffer.set_pos(buffer.pos() + literal.ends_with().len());

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
