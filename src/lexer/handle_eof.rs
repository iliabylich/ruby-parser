use crate::lexer::Lexer;
use crate::token::{Loc, Token, TokenValue};

impl<'a> Lexer<'a> {
    pub(crate) fn handle_eof(&mut self) -> Option<Token> {
        match self.current_byte() {
            // EOF | NULL      | ^D         | ^Z
            None | Some(b'\0' | 0x04 | 0x1a) => {
                Some(Token(TokenValue::tEOF, Loc(self.pos(), self.pos())))
            }
            _ => None,
        }
    }
}
