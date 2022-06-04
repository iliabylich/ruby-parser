mod handle_interpolation;
pub(crate) use handle_interpolation::handle_interpolation;

mod handle_eof;
pub(crate) use handle_eof::handle_eof;

mod handle_interpolation_end;
pub(crate) use handle_interpolation_end::handle_interpolation_end;

mod handle_string_end;
pub(crate) use handle_string_end::handle_string_end;

mod handle_processed_string_content;
pub(crate) use handle_processed_string_content::handle_processed_string_content;

use crate::token::{token, Token};

fn string_content_to_emit(start: usize, end: usize) -> Option<Token> {
    if start == end {
        None
    } else {
        Some(token!(tSTRING_CONTENT, start, end))
    }
}
