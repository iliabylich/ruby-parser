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
        &self.input[start..end]
    }
}
