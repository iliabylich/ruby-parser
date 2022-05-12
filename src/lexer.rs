use crate::token::{BinOp, Token};

pub struct Lexer<'a> {
    input: &'a [u8],
    pos: usize,
    current_token: Token<'a>,
    debug: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            input: s.as_bytes(),
            pos: 0,
            current_token: Token::None,
            debug: false,
        }
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    pub fn current_token(&self) -> Token<'a> {
        self.current_token
    }

    pub fn get_next_token(&mut self) {
        // skip whitespaces
        while self.current_byte() == Some(b' ') {
            self.pos += 1;
        }

        let token = match self.current_byte() {
            None => Token::tEOF,
            Some(b'+') => {
                self.pos += 1;
                Token::BinOp(BinOp::tPLUS)
            }
            Some(b'-') => {
                self.pos += 1;
                Token::BinOp(BinOp::tMINUS)
            }
            Some(b'*') => {
                self.pos += 1;
                match self.current_byte() {
                    Some(b'*') => {
                        self.pos += 1;
                        Token::BinOp(BinOp::tPOW)
                    }
                    _ => Token::BinOp(BinOp::tSTAR),
                }
            }
            Some(b'/') => {
                self.pos += 1;
                Token::BinOp(BinOp::tDIVIDE)
            }
            Some(b'(') => {
                self.pos += 1;
                Token::tLPAREN
            }
            Some(b')') => {
                self.pos += 1;
                Token::tRPAREN
            }
            Some(byte) if byte.is_ascii_digit() => {
                let start = self.pos;
                self.pos += 1;
                while let Some(byte) = self.current_byte() {
                    if !byte.is_ascii_digit() {
                        break;
                    }
                    self.pos += 1;
                }
                let num = &self.input[start..self.pos];
                // SAFETY: all bytes in num are ASCII digits
                let num = unsafe { std::str::from_utf8_unchecked(num) };
                Token::tINTEGER(num)
            }
            Some(byte) => Token::Error(byte as char),
        };

        if self.debug {
            println!("Reading token {:?}", token);
        }
        self.current_token = token
    }

    fn current_byte(&mut self) -> Option<u8> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
    }
}
