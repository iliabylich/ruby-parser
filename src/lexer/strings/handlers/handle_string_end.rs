use std::ops::ControlFlow;

use crate::{
    buffer::{BufferWithCursor, Pattern},
    lexer::strings::{action::StringExtendAction, handlers::handle_processed_string_content},
    loc::loc,
    token::token,
};

pub(crate) fn handle_string_end<P>(
    buffer: &mut BufferWithCursor,
    start: usize,
    starts_with: P,
    ends_with: P,
    ends_with_nesting: &mut usize,
) -> ControlFlow<StringExtendAction>
where
    P: Pattern,
{
    if buffer.lookahead(&ends_with) {
        if *ends_with_nesting == 0 {
            // match! actual string end
            handle_processed_string_content(buffer.for_lookahead(), start, buffer.pos())?;

            let start = buffer.pos();
            let end = start + ends_with.length();
            buffer.set_pos(end);

            return ControlFlow::Break(StringExtendAction::FoundStringEnd {
                token: token!(tSTRING_END, loc!(start, end)),
            });
        } else {
            // just a part of the string content like
            // %Q{ {} }
            //      ^
            *ends_with_nesting -= 1;
            return ControlFlow::Continue(());
        }
    }

    // track occurrence of `starts_with` byte to handle cases like
    // %Q{ {} }
    //     ^^
    if buffer.lookahead(&starts_with) {
        *ends_with_nesting += 1;
    }

    ControlFlow::Continue(())
}
