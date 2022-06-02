pub(crate) mod atmark;
pub(crate) mod buffer;
pub(crate) mod gvar;
pub(crate) mod handle_eof;
pub(crate) mod ident;
pub(crate) mod numbers;
pub(crate) mod percent;
pub(crate) mod punctuation;
pub(crate) mod skip_ws;
pub(crate) mod strings;

use crate::token::{token, Token};
use atmark::parse_atmark;
use buffer::Buffer;
use gvar::parse_gvar;
use ident::parse_ident;
use numbers::parse_number;
use percent::parse_percent;
use strings::parse_string;

use strings::{action::StringExtendAction, literal::StringLiteral, stack::StringLiteralStack};

pub struct Lexer<'a> {
    pub(crate) buffer: Buffer<'a>,
    debug: bool,

    string_literals: StringLiteralStack<'a>,

    current_token: Option<Token>,

    curly_nest: usize,
    paren_nest: usize,
    brack_nest: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            buffer: Buffer::new(input),
            debug: false,

            string_literals: StringLiteralStack::new(),

            current_token: None,

            curly_nest: 0,
            paren_nest: 0,
            brack_nest: 0,
        }
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    pub fn current_token(&mut self) -> Token {
        if self.current_token.is_none() {
            self.current_token = Some(self.next_token());
        }
        // SAFETY: we've filled in current_token above
        //         it's guaranteed to hold Some(Token)
        unsafe { self.current_token.unwrap_unchecked() }
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
        token
    }

    #[cfg(test)]
    pub(crate) fn tokenize_until_eof(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        loop {
            let token = self.next_token();
            tokens.push(token);
            if token.value() == crate::token::TokenValue::tEOF {
                break;
            }
        }
        tokens
    }

    pub(crate) fn skip_token(&mut self) {
        self.current_token = None;
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
            StringExtendAction::EmitEOF => {
                // close current literal
                self.string_literals.pop();
                // and emit EOF
                token!(tEOF, self.buffer.pos(), self.buffer.pos())
            }
        }
    }

    pub fn tokenize_normally(&mut self) -> Token {
        if let Some(eof_t) = self.handle_eof() {
            return eof_t;
        }
        self.skip_ws();

        let start = self.pos();

        // Test token for testing.
        // It allows sub-component tests to not depend on other components
        #[cfg(test)]
        if self.buffer.const_lookahead(b"TEST_TOKEN") {
            let end = start + "TEST_TOKEN".len();
            self.buffer.set_pos(end);
            return token!(tTEST_TOKEN, start, end);
        }

        // SAFETY: None (i.e. EOF) has been handled above in `handle_eof`.
        //         so `.unwrap_unchecked()` is safe
        let byte = unsafe { self.current_byte().unwrap_unchecked() };

        match byte {
            b'#' => OnByte::<b'#'>::on_byte(self),
            b'\n' => {
                // TODO: handle NL
                self.skip_byte();
                self.tokenize_normally()
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
                parse_gvar(&mut self.buffer)
            }
            b'@' => {
                self.buffer.set_pos(start);
                parse_atmark(&mut self.buffer)
            }
            b'_' => OnByte::<b'_'>::on_byte(self),

            _ident_start => {
                self.buffer.set_pos(start);
                parse_ident(&mut self.buffer)
            }
        }
    }

    pub(crate) fn tokenize_heredoc_id(&mut self) -> Option<&'a [u8]> {
        None
    }
}

pub(crate) trait OnByte<'a, const BYTE: u8> {
    fn on_byte(&mut self) -> Token;
}

macro_rules! assert_lex {
    (
        $test_name:ident,
        $input:expr,
        $tok:expr,
        $value:expr,
        $loc:expr,
        setup = $pre:expr,
        assert = $assert:expr
    ) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            use crate::{
                lexer::Lexer,
                token::{Loc, TokenValue::*},
            };
            let mut lexer = Lexer::new($input);
            $pre(&mut lexer);
            let token = lexer.current_token();
            assert_eq!(token.value(), $tok);
            assert_eq!(token.loc(), Loc($loc.start, $loc.end));
            assert_eq!(token.loc().end(), lexer.buffer.pos());
            assert_eq!(&$input[$loc.start..$loc.end], $value);
            $assert(&lexer);
        }
    };
    // Shortcut with no lexer setup/extra assert
    ($test_name:ident, $input:expr, $tok:expr, $value:expr, $loc:expr) => {
        assert_lex!(
            $test_name,
            $input,
            $tok,
            $value,
            $loc,
            setup = |_lexer: &mut Lexer| {},
            assert = |_lexer: &Lexer| {}
        );
    };
}
pub(crate) use assert_lex;
