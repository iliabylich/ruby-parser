mod handle_interpolation;
pub(crate) use handle_interpolation::handle_interpolation;

mod handle_eof;
pub(crate) use handle_eof::handle_eof;

mod handle_interpolation_end;
pub(crate) use handle_interpolation_end::handle_interpolation_end;

mod handle_next_action;
pub(crate) use handle_next_action::handle_next_action;

mod handle_string_end;
pub(crate) use handle_string_end::handle_string_end;

use crate::token::{token, Token};

fn string_content_to_emit(start: usize, end: usize) -> Option<Token> {
    if start == end {
        None
    } else {
        Some(token!(tSTRING_CONTENT, start, end))
    }
}

pub(crate) mod contracts {
    use crate::lexer::strings::action::NextAction;

    pub(crate) trait HasNextAction {
        fn next_action_mut(&mut self) -> &mut NextAction;
    }

    pub(crate) trait HasInterpolation {
        fn currently_in_interpolation(&self) -> bool;
        fn currently_in_interpolation_mut(&mut self) -> &mut bool;

        fn supports_interpolation(&self) -> bool;

        fn interpolation_started_with_curly_level(&self) -> usize;
    }
}
