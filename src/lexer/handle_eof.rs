use crate::lexer::Lexer;
use crate::token::{token, Token};

impl<'a> Lexer<'a> {
    pub(crate) fn handle_eof(&mut self) -> Option<Token<'a>> {
        match self.current_byte() {
            // EOF | NULL      | ^D         | ^Z
            None | Some(b'\0' | 0x04 | 0x1a) => Some(token!(tEOF, self.pos(), self.pos())),
            _ => None,
        }
    }
}
