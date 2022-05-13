mod buffer;
mod handle_eof;
mod skip_ws;
mod string_literals;

use crate::token::{Loc, Token, TokenValue};
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
        self.handle_eof()?;
        self.skip_ws();

        let start = self.pos();

        // SAFETY: None (i.e. EOF) has been handled above, so `.unwrap_unchecked()` is safe
        let token = match unsafe { self.take_byte().unwrap_unchecked() } {
            b'#' => {
                todo!("handle comment")
            }
            b'*' => {
                if let Some(b'*') = self.current_byte() {
                    self.skip_byte();
                    if let Some(b'=') = self.current_byte() {
                        self.skip_byte();
                        Token(TokenValue::tOP_ASGN(b"**="), Loc(start, self.pos()))
                    } else {
                        Token(TokenValue::tPOW, Loc(start, self.pos()))
                    }
                } else if let Some(b'=') = self.current_byte() {
                    self.skip_byte();
                    Token(TokenValue::tOP_ASGN(b"*="), Loc(start, self.pos()))
                } else {
                    Token(TokenValue::tSTAR, Loc(start, self.pos()))
                }
            }
            b'!' => {
                // !@ is handled on the parser level
                if let Some(b'=') = self.current_byte() {
                    self.skip_byte();
                    Token(TokenValue::tNEQ, Loc(start, self.pos()))
                } else if let Some(b'~') = self.current_byte() {
                    self.skip_byte();
                    Token(TokenValue::tNMATCH, Loc(start, self.pos()))
                } else {
                    Token(TokenValue::tBANG, Loc(start, self.pos()))
                }
            }
            b'=' => {
                if self.buffer.lookahead(b"begin") {
                    Token(TokenValue::tEMBEDDED_COMMENT_START, Loc(start, self.pos()))
                } else if let Some(b'=') = self.current_byte() {
                    self.skip_byte();
                    if let Some(b'=') = self.current_byte() {
                        self.skip_byte();
                        Token(TokenValue::tEQQ, Loc(start, self.pos()))
                    } else {
                        Token(TokenValue::tEQ, Loc(start, self.pos()))
                    }
                } else if let Some(b'~') = self.current_byte() {
                    self.skip_byte();
                    Token(TokenValue::tMATCH, Loc(start, self.pos()))
                } else if let Some(b'>') = self.current_byte() {
                    self.skip_byte();
                    Token(TokenValue::tASSOC, Loc(start, self.pos()))
                } else {
                    Token(TokenValue::tEQL, Loc(start, self.pos()))
                }
            }
            b'<' => todo!(),
            b'>' => todo!(),
            b'"' => todo!(),
            b'`' => todo!(),
            b'\'' => todo!(),
            b'?' => todo!(),
            b'&' => todo!(),
            b'|' => todo!(),

            // todo: extend
            b'+' => Token(TokenValue::tPLUS, Loc(start, self.pos())),
            b'-' => Token(TokenValue::tMINUS, Loc(start, self.pos())),

            b'.' => todo!(),
            b'0'..=b'9' => {
                // todo: parse numeric
                while let Some(b'0'..=b'9') = self.current_byte() {
                    self.skip_byte();
                }
                let num = self.slice(start, self.pos());
                Token(TokenValue::tINTEGER(num), Loc(start, self.pos()))
            }

            b')' => Token(TokenValue::tRPAREN, Loc(start, self.pos())),
            b']' => Token(TokenValue::tRBRACK, Loc(start, self.pos())),
            b'}' => Token(TokenValue::tRCURLY, Loc(start, self.pos())),

            b':' => todo!(),

            b'/' => Token(TokenValue::tDIVIDE, Loc(start, self.pos())),
            b'^' => todo!(),
            b';' => todo!(),
            b',' => todo!(),
            b'~' => todo!(),
            b'(' => Token(TokenValue::tLPAREN, Loc(start, self.pos())),
            b'[' => Token(TokenValue::tLBRACK, Loc(start, self.pos())),
            b'{' => Token(TokenValue::tLCURLY, Loc(start, self.pos())),
            b'\\' => todo!(),
            b'%' => todo!(),
            b'$' => todo!(),
            b'@' => todo!(),
            b'_' => todo!(),

            byte => {
                // TODO: parse ident
                Token(TokenValue::Error(byte as char), Loc(start, self.pos()))
            }
        };

        self.add_token(token);
        Ok(())
    }
}

#[cfg(test)]
mod lexer_tests;
