mod buffer;
pub(crate) use buffer::Buffer;

mod buffer_with_cursor;
pub(crate) use buffer_with_cursor::BufferWithCursor;

mod pattern;
pub(crate) use pattern::Pattern;

pub(crate) mod utf8;

mod scan_while_matches_pattern;
pub(crate) use scan_while_matches_pattern::{scan_while_matches_pattern, LookaheadResult};
