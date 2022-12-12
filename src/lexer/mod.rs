pub(crate) mod atmark;
pub(crate) mod gvar;
pub(crate) mod handle_eof;
pub(crate) mod heredoc_id;
pub(crate) mod ident;
pub(crate) mod numbers;
pub(crate) mod percent;
pub(crate) mod punctuation;
pub(crate) mod qmark;
pub(crate) mod skip_ws;
pub(crate) mod strings;

use atmark::AtMark;
use gvar::Gvar;
use ident::Ident;
use numbers::parse_number;
use percent::parse_percent;
use strings::parse_string;

use crate::{
    buffer::BufferWithCursor, lexer::strings::stack::StringLiteralStack, loc::loc, token::token,
    Token, TokenKind,
};
use strings::{action::StringExtendAction, literal::StringLiteral};

#[derive(Debug)]
pub struct Lexer {
    debug: bool,

    pub(crate) buffer: BufferWithCursor,
    pub(crate) required_new_expr: bool,

    pub(crate) string_literals: StringLiteralStack,

    pub(crate) curly_nest: usize,
    pub(crate) paren_nest: usize,
    pub(crate) brack_nest: usize,

    pub(crate) tokens: Vec<Token>,
    pub(crate) token_idx: usize,

    pub(crate) seen_whitespace: bool,
    pub(crate) seen_nl: bool,
}

impl Lexer {
    pub(crate) fn new(input: &[u8]) -> Self {
        Self {
            debug: false,

            buffer: BufferWithCursor::new(input),
            required_new_expr: false,

            string_literals: StringLiteralStack::new(),

            curly_nest: 0,
            paren_nest: 0,
            brack_nest: 0,

            tokens: vec![],
            token_idx: 0,

            seen_whitespace: false,
            seen_nl: false,
        }
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    pub fn current_token(&mut self) -> Token {
        if let Some(current_token) = self.tokens.get(self.token_idx) {
            *current_token
        } else {
            self.seen_whitespace = false;
            self.seen_nl = false;

            // get new token until we find not-whitespace token
            loop {
                let token = self.next_token();
                self.tokens.push(token);

                match token.kind {
                    TokenKind::tWHITESPACE => {
                        self.seen_whitespace = true;
                        self.token_idx += 1;
                    }
                    TokenKind::tNL => {
                        self.seen_nl = true;
                        self.token_idx += 1;
                    }
                    _ => {
                        return token;
                    }
                }
            }
        }
    }

    fn next_token(&mut self) -> Token {
        let token = if self.string_literals.last().is_some() {
            self.tokenize_while_in_string()
        } else {
            self.tokenize_normally()
        };
        if self.debug {
            println!("Returning token {:?}", token);
        }

        // Reset one-time flag
        self.required_new_expr = false;

        token
    }

    pub(crate) fn lookahead_is_identifier(&self) -> bool {
        Ident::lookahead(self.buffer.for_lookahead(), self.buffer.pos()).is_some()
    }

    #[cfg(test)]
    pub(crate) fn tokenize_until_eof(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        loop {
            let token = self.next_token();
            let is_eof = token.is(crate::token::TokenKind::tEOF);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        tokens
    }

    #[allow(dead_code)]
    pub(crate) fn require_new_expr(&mut self) {
        self.required_new_expr = true;
    }

    pub(crate) fn skip_token(&mut self) {
        self.token_idx += 1;
    }

    fn tokenize_while_in_string(&mut self) -> Token {
        // SAFETY: this method is called only if `string_literals` has at least 1 item
        let literal = unsafe { self.string_literals.last_mut().unwrap_unchecked() };

        match parse_string(literal, &mut self.buffer, self.curly_nest) {
            StringExtendAction::EmitToken { token } => {
                // just emit what literal gives us
                token
            }
            StringExtendAction::FoundStringEnd { token } => {
                // close current literal
                self.string_literals.pop();
                // and dispatch string end token
                token
            }
            StringExtendAction::FoundInterpolation { token } => {
                // dispatch dynamic string begin token
                token
            }
            StringExtendAction::ReadInterpolatedContent => {
                // we are after `#{` (but not at matching '}')
                // and should read an interpolated value
                self.tokenize_normally()
            }
            StringExtendAction::EmitEOF { at: eof_pos } => {
                // close current literal
                self.string_literals.pop();
                // and emit EOF
                token!(tEOF, loc!(eof_pos, eof_pos))
            }
        }
    }

    pub fn tokenize_normally(&mut self) -> Token {
        if let Some(eof_t) = self.handle_eof() {
            return eof_t;
        }
        if let Some(sp_t) = self.skip_ws() {
            return sp_t;
        }

        let start = self.buffer.pos();

        // Test token for testing.
        // It allows sub-component tests to not depend on other components
        #[cfg(test)]
        if self.buffer.lookahead(b"TEST_TOKEN") {
            let end = start + "TEST_TOKEN".len();
            self.buffer.set_pos(end);
            return token!(tTEST_TOKEN, loc!(start, end));
        }

        // SAFETY: None (i.e. EOF) has been handled above in `handle_eof`.
        //         so `.unwrap_unchecked()` is safe
        let byte = unsafe { self.buffer.current_byte().unwrap_unchecked() };

        match byte {
            b'#' => OnByte::<b'#'>::on_byte(self),
            b'\n' => {
                let token = token!(tNL, loc!(self.buffer.pos(), self.buffer.pos() + 1));
                self.buffer.skip_byte();
                token
            }
            b'*' => OnByte::<b'*'>::on_byte(self),
            b'!' => OnByte::<b'!'>::on_byte(self),
            b'=' => OnByte::<b'='>::on_byte(self),
            b'<' => OnByte::<b'<'>::on_byte(self),
            b'>' => OnByte::<b'>'>::on_byte(self),
            b'"' => OnByte::<b'"'>::on_byte(self),
            b'`' => OnByte::<b'`'>::on_byte(self),
            b'\'' => OnByte::<b'\''>::on_byte(self),
            b'?' => OnByte::<b'?'>::on_byte(self),
            b'&' => OnByte::<b'&'>::on_byte(self),
            b'|' => OnByte::<b'|'>::on_byte(self),
            b'+' => OnByte::<b'+'>::on_byte(self),
            b'-' => OnByte::<b'-'>::on_byte(self),
            b'.' => OnByte::<b'.'>::on_byte(self),
            b'0'..=b'9' => {
                self.buffer.set_pos(start);
                parse_number(&mut self.buffer)
            }

            b')' => OnByte::<b')'>::on_byte(self),
            b']' => OnByte::<b']'>::on_byte(self),
            b'}' => OnByte::<b'}'>::on_byte(self),

            b':' => OnByte::<b':'>::on_byte(self),

            b'/' => OnByte::<b'/'>::on_byte(self),
            b'^' => OnByte::<b'^'>::on_byte(self),
            b';' => OnByte::<b';'>::on_byte(self),
            b',' => OnByte::<b','>::on_byte(self),
            b'~' => OnByte::<b'~'>::on_byte(self),
            b'(' => OnByte::<b'('>::on_byte(self),
            b'[' => OnByte::<b'['>::on_byte(self),
            b'{' => OnByte::<b'{'>::on_byte(self),
            b'\\' => OnByte::<b'\\'>::on_byte(self),
            b'%' => {
                self.buffer.set_pos(start);
                let (literal, token) = parse_percent(&mut self.buffer, self.curly_nest);
                if let Some(literal) = literal {
                    self.string_literals.push(literal);
                }
                token
            }
            b'$' => {
                self.buffer.set_pos(start);
                Gvar::parse(&mut self.buffer)
            }
            b'@' => {
                self.buffer.set_pos(start);
                AtMark::parse(&mut self.buffer)
            }
            b'_' => OnByte::<b'_'>::on_byte(self),

            _ident_start => {
                self.buffer.set_pos(start);
                Ident::parse(&mut self.buffer)
            }
        }
    }
}

pub(crate) trait OnByte<const BYTE: u8> {
    fn on_byte(&mut self) -> Token;
}
