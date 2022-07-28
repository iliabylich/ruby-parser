use crate::{
    lexer::Lexer,
    loc::loc,
    token::{token, Token},
};

impl Lexer {
    pub(crate) fn handle_eof(&mut self) -> Option<Token> {
        match self.buffer().current_byte() {
            // EOF | NULL      | ^D         | ^Z
            None | Some(b'\0' | 0x04 | 0x1a) => {
                Some(token!(tEOF, loc!(self.buffer().pos(), self.buffer().pos())))
            }
            _ => None,
        }
    }
}
