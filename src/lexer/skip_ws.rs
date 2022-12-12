use crate::{lexer::Lexer, loc::loc, token::token, Token};

impl Lexer {
    pub(crate) fn skip_ws(&mut self) -> Option<Token> {
        let start = self.buffer.pos();

        loop {
            match self.buffer.current_byte() {
                // whitespaces
                Some(b'\r') => {
                    // TODO: warn about \r at middle of the line
                    self.buffer.skip_byte();
                    continue;
                }

                // SPACE  | TAB   | LF   | VTAB
                Some(b' ' | b'\t' | 0x0c | 0x0b) => {
                    self.buffer.skip_byte();
                    continue;
                }

                _ => break,
            }
        }

        let end = self.buffer.pos();

        if start == end {
            None
        } else {
            Some(token!(tWHITESPACE, loc!(start, end)))
        }
    }
}
