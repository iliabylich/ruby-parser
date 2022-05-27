mod lexer_proxy;
pub(crate) mod utf8;

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

    pub(crate) fn lookahead(&self, pattern: &[u8]) -> bool {
        if let Some(slice) = self.input.get(self.pos..self.pos + pattern.len()) {
            slice == pattern
        } else {
            false
        }
    }

    pub(crate) fn const_lookahead<const N: usize>(&self, pattern: &[u8; N]) -> bool {
        for i in 0..N {
            if self.byte_at(self.pos + i) != Some(pattern[i]) {
                return false;
            }
        }
        true
    }
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
fn test_const_lookahead() {
    let buffer = Buffer::new(b"foo");
    assert!(buffer.const_lookahead(b"f"));
    assert!(buffer.const_lookahead(b"fo"));
    assert!(buffer.const_lookahead(b"foo"));
    assert!(!buffer.const_lookahead(b"fooo"));
}
