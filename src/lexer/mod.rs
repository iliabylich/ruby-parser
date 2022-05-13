mod buffer;
mod handle_eof;
mod skip_ws;
mod string_literals;

use crate::token::{Loc, Token, TokenValue};
use buffer::Buffer;
use string_literals::{StringLiteral, StringLiteralAction, StringLiteralStack};

pub struct Lexer<'a> {
    buffer: Buffer<'a>,
    debug: bool,

    string_literals: StringLiteralStack<'a>,

    tokens: Vec<Token<'a>>,
    token_idx: usize,

    curly_braces: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            buffer: Buffer::new(s.as_bytes()),
            debug: false,

            string_literals: StringLiteralStack::new(),

            tokens: vec![],
            token_idx: 0,

            curly_braces: 0,
        }
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    pub fn tokenize_until_eof(&mut self) {
        if let Err(_err) = self._tokenize() {
            // TODO: handle unexpected EOF error
        }
    }

    fn _tokenize(&mut self) -> Result<(), ()> {
        loop {
            if let Some(literal) = self.string_literals.last() {
                self.tokenize_while_in_string(literal)?
            } else {
                self.tokenize_normally()?
            };
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

    fn tokenize_while_in_string(&mut self, literal: StringLiteral<'a>) -> Result<(), ()> {
        match literal.lex(&mut self.buffer) {
            StringLiteralAction::InInterpolation {
                interpolation_started_with_curly_level,
            } => {
                if self.current_byte() == Some(b'}')
                    && interpolation_started_with_curly_level == self.curly_braces
                {
                    self.add_token(Token(TokenValue::tRCURLY, Loc(self.pos(), self.pos() + 1)));
                    self.skip_byte();
                } else {
                    // we are after `#{` and should read an interpolated value
                    self.tokenize_normally()?;
                }
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
        Ok(())
    }

    pub fn tokenize_normally(&mut self) -> Result<(), ()> {
        self.handle_eof()?;
        self.skip_ws();

        let start = self.pos();

        // SAFETY: None (i.e. EOF) has been handled above in `handle_eof`.
        //         so `.unwrap_unchecked()` is safe
        match unsafe { self.take_byte().unwrap_unchecked() } {
            b'#' => OnByte::<b'#'>::on_byte(self)?,
            b'*' => OnByte::<b'*'>::on_byte(self)?,
            b'!' => OnByte::<b'!'>::on_byte(self)?,
            b'=' => OnByte::<b'='>::on_byte(self)?,
            b'<' => todo!(),
            b'>' => todo!(),
            b'"' => todo!(),
            b'`' => todo!(),
            b'\'' => todo!(),
            b'?' => todo!(),
            b'&' => todo!(),
            b'|' => todo!(),
            b'+' => OnByte::<b'+'>::on_byte(self)?,
            b'-' => OnByte::<b'-'>::on_byte(self)?,
            b'.' => todo!(),
            b'0'..=b'9' => {
                // todo: parse numeric
                while let Some(b'0'..=b'9') = self.current_byte() {
                    self.skip_byte();
                }
                let num = self.slice(start, self.pos());
                self.add_token(Token(TokenValue::tINTEGER(num), Loc(start, self.pos())))
            }

            b')' => self.add_token(Token(TokenValue::tRPAREN, Loc(start, self.pos()))),
            b']' => self.add_token(Token(TokenValue::tRBRACK, Loc(start, self.pos()))),
            b'}' => self.add_token(Token(TokenValue::tRCURLY, Loc(start, self.pos()))),

            b':' => todo!(),

            b'/' => self.add_token(Token(TokenValue::tDIVIDE, Loc(start, self.pos()))),
            b'^' => todo!(),
            b';' => todo!(),
            b',' => todo!(),
            b'~' => todo!(),
            b'(' => self.add_token(Token(TokenValue::tLPAREN, Loc(start, self.pos()))),
            b'[' => self.add_token(Token(TokenValue::tLBRACK, Loc(start, self.pos()))),
            b'{' => self.add_token(Token(TokenValue::tLCURLY, Loc(start, self.pos()))),
            b'\\' => todo!(),
            b'%' => todo!(),
            b'$' => todo!(),
            b'@' => todo!(),
            b'_' => todo!(),

            byte => {
                // TODO: parse ident
                self.add_token(Token(
                    TokenValue::Error(byte as char),
                    Loc(start, self.pos()),
                ))
            }
        };

        Ok(())
    }
}

trait OnByte<const BYTE: u8> {
    fn on_byte(&mut self) -> Result<(), ()>;
}

impl OnByte<b'#'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        todo!("handle comment");
        #[allow(unreachable_code)]
        Err(())
    }
}

impl OnByte<b'*'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;
        let token = if let Some(b'*') = self.current_byte() {
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
        };
        self.add_token(token);
        Ok(())
    }
}

impl OnByte<b'!'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;

        // !@ is handled on the parser level
        let token = if let Some(b'=') = self.current_byte() {
            self.skip_byte();
            Token(TokenValue::tNEQ, Loc(start, self.pos()))
        } else if let Some(b'~') = self.current_byte() {
            self.skip_byte();
            Token(TokenValue::tNMATCH, Loc(start, self.pos()))
        } else {
            Token(TokenValue::tBANG, Loc(start, self.pos()))
        };
        self.add_token(token);
        Ok(())
    }
}

impl OnByte<b'='> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        let start = self.pos() - 1;

        let token = if self.buffer.lookahead(b"begin") {
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
        };
        self.add_token(token);
        Ok(())
    }
}

impl OnByte<b'+'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        // TODO: extend
        let start = self.pos() - 1;
        self.add_token(Token(TokenValue::tPLUS, Loc(start, self.pos())));
        Ok(())
    }
}

impl OnByte<b'-'> for Lexer<'_> {
    fn on_byte(&mut self) -> Result<(), ()> {
        // TODO: extend
        let start = self.pos() - 1;
        self.add_token(Token(TokenValue::tMINUS, Loc(start, self.pos())));
        Ok(())
    }
}

#[cfg(test)]
mod lexer_tests;
