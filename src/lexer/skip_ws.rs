use crate::lexer::Lexer;

impl Lexer {
    pub(crate) fn skip_ws(&mut self) {
        loop {
            match self.current_byte() {
                // whitespaces
                Some(b'\r') => {
                    // TODO: warn about \r at middle of the line
                    self.skip_byte();
                    continue;
                }

                // SPACE  | TAB   | LF   | VTAB
                Some(b' ' | b'\t' | 0x0c | 0x0b) => {
                    self.skip_byte();
                    continue;
                }

                _ => break,
            }
        }
    }
}
