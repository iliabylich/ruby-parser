use crate::lexer::Lexer;
// buffer shortcut delegators
impl<'a> Lexer<'a> {
    pub(crate) fn skip_byte(&mut self) {
        self.buffer.skip_byte()
    }
    pub(crate) fn current_byte(&self) -> Option<u8> {
        self.buffer.current_byte()
    }
    pub(crate) fn pos(&self) -> usize {
        self.buffer.pos()
    }
    #[allow(dead_code)]
    pub(crate) fn slice(&self, start: usize, end: usize) -> &'a [u8] {
        self.buffer.slice(start, end)
    }
}
