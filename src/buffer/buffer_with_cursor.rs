use crate::buffer::{Buffer, Pattern};

pub struct BufferWithCursor {
    buffer: Buffer,
    pos: usize,
}

impl BufferWithCursor {
    pub(crate) fn new(input: &[u8]) -> Self {
        Self {
            buffer: Buffer::new(input),
            pos: 0,
        }
    }

    // Delegators
    pub(crate) fn slice(&self, start: usize, end: usize) -> Option<&[u8]> {
        self.buffer.slice(start, end)
    }
    pub(crate) fn byte_at(&self, idx: usize) -> Option<u8> {
        self.buffer.byte_at(idx)
    }

    // Getter for lookahead
    pub(crate) fn for_lookahead(&self) -> &Buffer {
        &self.buffer
    }

    // Getter for mutable lookahead
    pub(crate) fn for_lookahead_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
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

    pub(crate) fn current_byte(&self) -> Option<u8> {
        self.buffer.byte_at(self.pos)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.current_byte().is_none()
    }

    pub(crate) fn lookahead<P>(&self, pattern: &P) -> bool
    where
        P: Pattern,
    {
        self.buffer.lookahead(self.pos, pattern)
    }
}

impl std::fmt::Debug for BufferWithCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let input = &self.buffer.bytes()[self.pos..];
        let input = &input[..std::cmp::min(input.len(), 10)];
        let input = std::str::from_utf8(input).unwrap();

        f.debug_struct("BufferWithCursor")
            .field("input", &input)
            .field("pos", &self.pos)
            .finish()
    }
}

#[test]
fn test_lookahead() {
    let buffer = BufferWithCursor::new(b"foo");
    assert!(buffer.lookahead(b"f"));
    assert!(buffer.lookahead(b"fo"));
    assert!(buffer.lookahead(b"foo"));
    assert!(!buffer.lookahead(b"fooo"));
}
