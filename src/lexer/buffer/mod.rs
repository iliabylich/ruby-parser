mod lexer_proxy;
mod pattern;
pub(crate) mod utf8;

pub(crate) use pattern::Pattern;

pub struct Buffer<'a> {
    input: &'a [u8],
    pos: usize,
}

impl std::fmt::Debug for Buffer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let input = &self.input[self.pos..];
        let input = &input[..std::cmp::min(input.len(), 10)];
        let input = std::str::from_utf8(input).unwrap();

        f.debug_struct("Buffer")
            .field("input", &input)
            .field("pos", &self.pos)
            .finish()
    }
}

impl<'a> Buffer<'a> {
    pub(crate) fn new(input: &'a [u8]) -> Self {
        Self { input, pos: 0 }
    }

    pub(crate) fn skip_byte(&mut self) {
        self.pos += 1;
    }

    pub(crate) fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub(crate) fn pos(&self) -> usize {
        self.pos
    }

    pub(crate) fn slice(&self, start: usize, end: usize) -> &'a [u8] {
        self.input.get(start..end).unwrap_or_else(|| {
            panic!(
                "wrong start/end given: {}..{} (lenth = {})",
                start, end, self.pos
            )
        })
    }

    pub(crate) fn byte_at(&self, idx: usize) -> Option<u8> {
        self.input.get(idx).map(|byte| *byte)
    }

    pub(crate) fn current_byte(&self) -> Option<u8> {
        self.byte_at(self.pos)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.current_byte().is_none()
    }

    pub(crate) fn lookahead<P>(&self, pattern: &P) -> bool
    where
        P: Pattern,
    {
        pattern.is_lookahead_of(self)
    }
}

macro_rules! scan_while_matches_pattern {
    ($buffer:expr, $start:expr, $pattern:pat) => {{
        use crate::lexer::buffer::LookaheadResult;

        let mut end = $start;
        loop {
            match $buffer.byte_at(end) {
                Some($pattern) => {
                    end += 1;
                }
                _ => {
                    break;
                }
            }
        }
        if ($start == end) {
            LookaheadResult::None
        } else {
            LookaheadResult::Some {
                length: end - $start,
            }
        }
    }};
}
pub(crate) use scan_while_matches_pattern;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum LookaheadResult {
    None,
    Some { length: usize },
}

pub(crate) trait Lookahead<'a> {
    type Output;

    fn lookahead(buffer: &Buffer<'a>, start: usize) -> Self::Output;
}

#[test]
fn test_lookahead() {
    let buffer = Buffer::new(b"foo");
    assert!(buffer.lookahead(b"f"));
    assert!(buffer.lookahead(b"fo"));
    assert!(buffer.lookahead(b"foo"));
    assert!(!buffer.lookahead(b"fooo"));
}

#[test]
fn test_scan_while_matches_pattern() {
    let buffer = Buffer::new(b"abcdefghijk");
    assert_eq!(
        scan_while_matches_pattern!(buffer, 0, b'a'..=b'd'),
        LookaheadResult::Some { length: 4 }
    );
    assert_eq!(
        scan_while_matches_pattern!(buffer, 0, b'a'..=b'z'),
        LookaheadResult::Some { length: 11 }
    );
    assert_eq!(
        scan_while_matches_pattern!(buffer, 0, b'0'..=b'9'),
        LookaheadResult::None
    );
}
