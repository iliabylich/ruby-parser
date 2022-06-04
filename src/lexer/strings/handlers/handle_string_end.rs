use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::{Buffer, Pattern},
        strings::{action::StringExtendAction, handlers::handle_processed_string_content},
    },
    token::token,
};

pub(crate) fn handle_string_end<'a, P>(
    ends_with: P,
    buffer: &mut Buffer<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction>
where
    P: Pattern,
{
    if buffer.lookahead(&ends_with) {
        handle_processed_string_content(start, buffer.pos())?;

        let start = buffer.pos();
        let end = start + ends_with.length();
        buffer.set_pos(end);

        ControlFlow::Break(StringExtendAction::FoundStringEnd {
            token: token!(tSTRING_END, start, end),
        })
    } else {
        ControlFlow::Continue(())
    }
}
