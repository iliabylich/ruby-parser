mod buffer;
mod handle_eof;
mod number;
mod punctuation;
mod skip_ws;
mod string_literals;

use crate::token::{Loc, Token, TokenValue};
use buffer::Buffer;
use number::parse_number;
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
        let byte = unsafe { self.take_byte().unwrap_unchecked() };

        match byte {
            b'#' => OnByte::<b'#'>::on_byte(self)?,
            b'*' => OnByte::<b'*'>::on_byte(self)?,
            b'!' => OnByte::<b'!'>::on_byte(self)?,
            b'=' => OnByte::<b'='>::on_byte(self)?,
            b'<' => OnByte::<b'<'>::on_byte(self)?,
            b'>' => OnByte::<b'>'>::on_byte(self)?,
            b'"' => OnByte::<b'"'>::on_byte(self)?,
            b'`' => OnByte::<b'`'>::on_byte(self)?,
            b'\'' => OnByte::<b'\''>::on_byte(self)?,
            b'?' => OnByte::<b'?'>::on_byte(self)?,
            b'&' => OnByte::<b'&'>::on_byte(self)?,
            b'|' => OnByte::<b'|'>::on_byte(self)?,
            b'+' => OnByte::<b'+'>::on_byte(self)?,
            b'-' => OnByte::<b'-'>::on_byte(self)?,
            b'.' => OnByte::<b'.'>::on_byte(self)?,
            b'0'..=b'9' => {
                self.buffer.set_pos(start);
                let token = parse_number(&mut self.buffer)?;
                self.add_token(token)
            }

            b')' => OnByte::<b')'>::on_byte(self)?,
            b']' => OnByte::<b']'>::on_byte(self)?,
            b'}' => OnByte::<b'}'>::on_byte(self)?,

            b':' => OnByte::<b':'>::on_byte(self)?,

            b'/' => OnByte::<b'/'>::on_byte(self)?,
            b'^' => OnByte::<b'^'>::on_byte(self)?,
            b';' => OnByte::<b';'>::on_byte(self)?,
            b',' => OnByte::<b','>::on_byte(self)?,
            b'~' => OnByte::<b'~'>::on_byte(self)?,
            b'(' => OnByte::<b'('>::on_byte(self)?,
            b'[' => OnByte::<b'['>::on_byte(self)?,
            b'{' => OnByte::<b'{'>::on_byte(self)?,
            b'\\' => OnByte::<b'\\'>::on_byte(self)?,
            b'%' => OnByte::<b'%'>::on_byte(self)?,
            b'$' => OnByte::<b'$'>::on_byte(self)?,
            b'@' => OnByte::<b'@'>::on_byte(self)?,
            b'_' => OnByte::<b'_'>::on_byte(self)?,

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

    pub(crate) fn tokenize_heredoc_id(&mut self) -> Option<&'a [u8]> {
        None
    }
}

pub(crate) trait OnByte<const BYTE: u8> {
    fn on_byte(&mut self) -> Result<(), ()>;
}

macro_rules! assert_lex {
    ($test_name:ident, $input:literal, $tok:expr, $loc:expr) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            use crate::{Lexer, Loc, TokenValue::*};
            let mut lexer = Lexer::new($input);
            lexer.tokenize_until_eof();
            assert_eq!(lexer.tokens[0].value(), $tok);
            assert_eq!(lexer.tokens[0].loc(), Loc($loc.start, $loc.end));
        }
    };
}
pub(crate) use assert_lex;
