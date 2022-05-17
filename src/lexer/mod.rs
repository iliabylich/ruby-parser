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

    current_token: Option<Token<'a>>,

    curly_nest: usize,
    paren_nest: usize,
    brack_nest: usize,
    new_expr_required: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            buffer: Buffer::new(s.as_bytes()),
            debug: false,

            string_literals: StringLiteralStack::new(),

            current_token: None,

            curly_nest: 0,
            paren_nest: 0,
            brack_nest: 0,
            new_expr_required: false,
        }
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    pub fn current_token(&mut self) -> Token<'a> {
        if self.current_token.is_none() {
            self.current_token = Some(self.next_token())
        }
        // SAFETY: we've filled in current_token above
        //         it's guaranteed to hold Some(Token)
        unsafe { self.current_token.unwrap_unchecked() }
    }

    fn next_token(&mut self) -> Token<'a> {
        if let Some(literal) = self.string_literals.last() {
            self.tokenize_while_in_string(literal)
        } else {
            self.tokenize_normally()
        }
    }

    pub(crate) fn require_new_expr(&mut self) {
        self.new_expr_required = true;
    }

    pub(crate) fn skip_token(&mut self) {
        self.current_token = None;
    }

    fn tokenize_while_in_string(&mut self, literal: StringLiteral<'a>) -> Token<'a> {
        match literal.lex(&mut self.buffer) {
            StringLiteralAction::InInterpolation {
                interpolation_started_with_curly_level,
            } => {
                if self.current_byte() == Some(b'}')
                    && interpolation_started_with_curly_level == self.curly_nest
                {
                    let token = Token(TokenValue::tRCURLY, Loc(self.pos(), self.pos() + 1));
                    self.skip_byte();
                    token
                } else {
                    // we are after `#{` and should read an interpolated value
                    self.tokenize_normally()
                }
            }
            StringLiteralAction::EmitStringContent {
                content,
                start,
                end,
            } => {
                let token = Token(TokenValue::tSTRING_CONTENT(content), Loc(start, end));
                self.buffer.set_pos(end);
                token
            }
            StringLiteralAction::CloseLiteral {
                content,
                start,
                end,
                jump_to,
            } => {
                let token = Token(TokenValue::tSTRING_END(content), Loc(start, end));
                self.buffer.set_pos(jump_to);
                self.string_literals.pop();
                token
            }
        }
    }

    pub fn tokenize_normally(&mut self) -> Token<'a> {
        if let Some(eof_t) = self.handle_eof() {
            return eof_t;
        }
        self.skip_ws();

        let start = self.pos();

        // SAFETY: None (i.e. EOF) has been handled above in `handle_eof`.
        //         so `.unwrap_unchecked()` is safe
        let byte = unsafe { self.take_byte().unwrap_unchecked() };

        match byte {
            b'#' => OnByte::<b'#'>::on_byte(self),
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
            b'%' => OnByte::<b'%'>::on_byte(self),
            b'$' => OnByte::<b'$'>::on_byte(self),
            b'@' => OnByte::<b'@'>::on_byte(self),
            b'_' => OnByte::<b'_'>::on_byte(self),

            byte => {
                // TODO: parse ident
                Token(TokenValue::Error(byte as char), Loc(start, self.pos()))
            }
        }
    }

    pub(crate) fn tokenize_heredoc_id(&mut self) -> Option<&'a [u8]> {
        None
    }
}

pub(crate) trait OnByte<'a, const BYTE: u8> {
    fn on_byte(&mut self) -> Token<'a>;
}

macro_rules! assert_lex {
    ($test_name:ident, $input:literal, $tok:expr, $loc:expr, setup = $pre:expr, assert = $assert:expr) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            use crate::{Lexer, Loc, TokenValue::*};
            let mut lexer = Lexer::new($input);
            $pre(&mut lexer);
            let token = lexer.current_token();
            assert_eq!(token.value(), $tok);
            assert_eq!(token.loc(), Loc($loc.start, $loc.end));
            $assert(&lexer);
        }
    };
    // Shortcut with no lexer setup/extra assert
    ($test_name:ident, $input:literal, $tok:expr, $loc:expr) => {
        assert_lex!(
            $test_name,
            $input,
            $tok,
            $loc,
            setup = |_lexer: &mut Lexer| {},
            assert = |_lexer: &Lexer| {}
        );
    };
}
pub(crate) use assert_lex;
