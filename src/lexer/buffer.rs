pub struct Buffer<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Buffer<'a> {
    pub(crate) fn new(input: &'a [u8]) -> Self {
        Self { input, pos: 0 }
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    pub(crate) fn current_byte(&self) -> Option<u8> {
        self.input.get(self.pos).map(|byte| *byte)
    }

    pub(crate) fn take_byte(&mut self) -> Option<u8> {
        let result = self.current_byte();
        self.skip_byte();
        result
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
        self.input.get(start..end).unwrap_or(b"")
    }

    pub(crate) fn lookahead(&self, pattern: &[u8]) -> bool {
        if let Some(slice) = self.input.get(self.pos..self.pos + pattern.len()) {
            slice == pattern
        } else {
            false
        }
    }

    pub(crate) fn byte_at(&self, idx: usize) -> Option<u8> {
        self.input.get(idx).map(|byte| *byte)
    }
}

use crate::Lexer;
// buffer shortcut delegators
impl<'a> Lexer<'a> {
    pub(crate) fn skip_byte(&mut self) {
        self.buffer.skip_byte()
    }
    pub(crate) fn current_byte(&self) -> Option<u8> {
        self.buffer.current_byte()
    }
    pub(crate) fn take_byte(&mut self) -> Option<u8> {
        self.buffer.take_byte()
    }
    #[allow(dead_code)]
    pub(crate) fn is_eof(&self) -> bool {
        self.buffer.is_eof()
    }
    pub(crate) fn pos(&self) -> usize {
        self.buffer.pos()
    }
    pub(crate) fn slice(&self, start: usize, end: usize) -> &'a [u8] {
        self.buffer.slice(start, end)
    }
}
