mod buffer;

use crate::token::{BinOp, Loc, Token, TokenValue};
use buffer::Buffer;

pub struct Lexer<'a> {
    buffer: Buffer<'a>,
    current_token: Token<'a>,
    debug: bool,
}

// buffer delegators
impl<'a> Lexer<'a> {
    pub(crate) fn skip_byte(&mut self) {
        self.buffer.skip_byte()
    }
    pub(crate) fn current_byte(&self) -> Option<u8> {
        self.buffer.current_byte()
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

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            buffer: Buffer::new(s.as_bytes()),
            current_token: Token::default(),
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
            self.skip_byte();
        }

        let start = self.pos();

        let token = match self.current_byte() {
            None => Token(TokenValue::tEOF, Loc(self.pos(), self.pos())),
            Some(b'+') => {
                self.skip_byte();
                Token(TokenValue::BinOp(BinOp::tPLUS), Loc(start, self.pos()))
            }
            Some(b'-') => {
                self.skip_byte();
                Token(TokenValue::BinOp(BinOp::tMINUS), Loc(start, self.pos()))
            }
            Some(b'*') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'*') => {
                        self.skip_byte();
                        Token(TokenValue::BinOp(BinOp::tPOW), Loc(start, self.pos()))
                    }
                    _ => Token(TokenValue::BinOp(BinOp::tSTAR), Loc(start, self.pos())),
                }
            }
            Some(b'/') => {
                self.skip_byte();
                Token(TokenValue::BinOp(BinOp::tDIVIDE), Loc(start, self.pos()))
            }
            Some(b'(') => {
                self.skip_byte();
                Token(TokenValue::tLPAREN, Loc(start, self.pos()))
            }
            Some(b')') => {
                self.skip_byte();
                Token(TokenValue::tRPAREN, Loc(start, self.pos()))
            }
            Some(b'=') => {
                self.skip_byte();
                match self.current_byte() {
                    Some(b'=') => {
                        self.skip_byte();
                        match self.current_byte() {
                            Some(b'=') => {
                                self.skip_byte();
                                Token(TokenValue::BinOp(BinOp::tEQQ), Loc(start, self.pos()))
                            }
                            _ => Token(TokenValue::BinOp(BinOp::tEQ), Loc(start, self.pos())),
                        }
                    }
                    _ => Token(TokenValue::BinOp(BinOp::tEQL), Loc(start, self.pos())),
                }
            }
            Some(byte) if byte.is_ascii_digit() => {
                let start = self.pos();
                self.skip_byte();
                while let Some(byte) = self.current_byte() {
                    if !byte.is_ascii_digit() {
                        break;
                    }
                    self.skip_byte();
                }
                let num = self.slice(start, self.pos());
                // SAFETY: all bytes in num are ASCII digits
                let num = unsafe { std::str::from_utf8_unchecked(num) };
                Token(TokenValue::tINTEGER(num), Loc(start, self.pos()))
            }
            Some(byte) => {
                self.skip_byte();
                Token(TokenValue::Error(byte as char), Loc(start, self.pos()))
            }
        };

        if self.debug {
            println!("Reading token {:?}", token);
        }
        self.current_token = token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_lex {
        ($name:ident, $input:literal, $tok:expr, $loc:expr) => {
            #[test]
            #[allow(non_snake_case)]
            fn $name() {
                let mut lexer = Lexer::new($input);
                lexer.get_next_token();
                assert_eq!(lexer.current_token.value(), $tok);
                assert_eq!(lexer.current_token.loc(), Loc($loc.start, $loc.end));
            }
        };
    }

    assert_lex!(tINTEGER, "42", TokenValue::tINTEGER("42"), 0..2);

    assert_lex!(BinOp_tPLUS, "+", TokenValue::BinOp(BinOp::tPLUS), 0..1);
    assert_lex!(BinOp_tMINUS, "-", TokenValue::BinOp(BinOp::tMINUS), 0..1);
    assert_lex!(BinOp_tSTAR, "*", TokenValue::BinOp(BinOp::tSTAR), 0..1);
    assert_lex!(BinOp_tDIVIDE, "/", TokenValue::BinOp(BinOp::tDIVIDE), 0..1);
    assert_lex!(BinOp_tPOW, "**", TokenValue::BinOp(BinOp::tPOW), 0..2);
    assert_lex!(BinOp_tEQL, "=", TokenValue::BinOp(BinOp::tEQL), 0..1);
    assert_lex!(BinOp_tEQ, "==", TokenValue::BinOp(BinOp::tEQ), 0..2);
    assert_lex!(BinOp_tEQQ, "===", TokenValue::BinOp(BinOp::tEQQ), 0..3);
}
