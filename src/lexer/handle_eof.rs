use crate::token::{Loc, Token, TokenValue};
use crate::Lexer;

impl<'a> Lexer<'a> {
    pub(crate) fn handle_eof(&mut self) -> Result<(), ()> {
        match self.current_byte() {
            // EOF | NULL      | ^D         | ^Z
            None | Some(b'\0' | 0x04 | 0x1a) => {
                let t_eof = Token(TokenValue::tEOF, Loc(self.pos(), self.pos()));
                self.add_token(t_eof);
                Err(())
            }
            _ => Ok(()),
        }
    }
}
