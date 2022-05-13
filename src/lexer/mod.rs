mod buffer;
mod string_literals;

use crate::token::{BinOp, Loc, Token, TokenValue};
use buffer::Buffer;
use string_literals::{StringLiteralAction, StringLiterals};

pub struct Lexer<'a> {
    buffer: Buffer<'a>,
    debug: bool,

    string_literals: StringLiterals<'a>,

    tokens: Vec<Token<'a>>,
    token_idx: usize,

    curly_braces: usize,
}

// buffer shortcut delegators
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
            debug: false,

            string_literals: StringLiterals::new(),

            tokens: vec![],
            token_idx: 0,

            curly_braces: 0,
        }
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    pub fn tokenize(&mut self) {
        loop {
            let got_some_tokens = self.get_next_token();
            if got_some_tokens.is_err() {
                break;
            }
        }
    }

    pub fn current_token(&self) -> Token<'a> {
        self.tokens[self.token_idx]
    }

    pub fn next_token(&mut self) {
        if self.token_idx < self.tokens.len() {
            self.token_idx += 1;
        }
    }

    pub(crate) fn add_token(&mut self, token: Token<'a>) {
        if self.debug {
            println!("Reading token {:?}", token);
        }

        self.tokens.push(token);
    }

    fn get_next_token(&mut self) -> Result<(), ()> {
        // Handle current string literal (if any)
        if let Some(string_literal) = self.string_literals.last() {
            match string_literal.lex(&mut self.buffer) {
                StringLiteralAction::InInterpolation {
                    interpolation_started_with_curly_level,
                } => {
                    if self.current_byte() == Some(b'}')
                        && interpolation_started_with_curly_level == self.curly_braces
                    {
                        self.add_token(Token(TokenValue::tRCURLY, Loc(self.pos(), self.pos() + 1)));
                        self.skip_byte();
                    }
                    // we are after `#{` and should read an interpolated value
                    self.get_next_value_token()?;
                }
                StringLiteralAction::EmitStringContent {
                    content,
                    start,
                    end,
                } => {
                    self.add_token(Token(TokenValue::tSTRING_CONTENT(content), Loc(start, end)));
                    self.buffer.set_pos(end);
                }
                StringLiteralAction::CloseLiteral {
                    content,
                    start,
                    end,
                    jump_to,
                } => {
                    self.add_token(Token(TokenValue::tSTRING_END(content), Loc(start, end)));
                    self.buffer.set_pos(jump_to);
                    self.string_literals.pop();
                }
            }
        } else {
            self.get_next_value_token()?
        }

        Ok(())
    }

    pub fn get_next_value_token(&mut self) -> Result<(), ()> {
        let start = self.pos();

        let token = match self.current_byte() {
            // EOF | NULL      | ^D         | ^Z
            None | Some(b'\0' | 0x04 | 0x1a) => {
                let t_eof = Token(TokenValue::tEOF, Loc(self.pos(), self.pos()));
                self.add_token(t_eof);
                return Err(());
            }

            // whitespaces
            Some(b'\r') => {
                // TODO: warn about \r at middle of the line
                return Ok(());
            }

            // SPACE  | TAB   | LF   | VTAB
            Some(b' ' | b'\t' | 0x0c | 0x0b) => {
                while let Some(b' ' | b'\t' | 0x0c | 0x0b) = self.current_byte() {
                    self.skip_byte()
                }
                return Ok(());
            }

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
                Token(TokenValue::tINTEGER(num), Loc(start, self.pos()))
            }
            Some(byte) => {
                self.skip_byte();
                Token(TokenValue::Error(byte as char), Loc(start, self.pos()))
            }
        };

        self.add_token(token);
        Ok(())
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
                lexer.tokenize();
                assert_eq!(lexer.tokens[0].value(), $tok);
                assert_eq!(lexer.tokens[0].loc(), Loc($loc.start, $loc.end));
            }
        };
    }

    assert_lex!(tINTEGER, "42", TokenValue::tINTEGER(b"42"), 0..2);

    assert_lex!(BinOp_tPLUS, "+", TokenValue::BinOp(BinOp::tPLUS), 0..1);
    assert_lex!(BinOp_tMINUS, "-", TokenValue::BinOp(BinOp::tMINUS), 0..1);
    assert_lex!(BinOp_tSTAR, "*", TokenValue::BinOp(BinOp::tSTAR), 0..1);
    assert_lex!(BinOp_tDIVIDE, "/", TokenValue::BinOp(BinOp::tDIVIDE), 0..1);
    assert_lex!(BinOp_tPOW, "**", TokenValue::BinOp(BinOp::tPOW), 0..2);
    assert_lex!(BinOp_tEQL, "=", TokenValue::BinOp(BinOp::tEQL), 0..1);
    assert_lex!(BinOp_tEQ, "==", TokenValue::BinOp(BinOp::tEQ), 0..2);
    assert_lex!(BinOp_tEQQ, "===", TokenValue::BinOp(BinOp::tEQQ), 0..3);
}
