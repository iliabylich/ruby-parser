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

#[cfg(test)]
use crate::state::OwnedState;
use crate::{
    loc::loc,
    state::{generate_state_ref_delegation, HasStateRef, StateRef},
    token::{token, Token},
};
use atmark::AtMark;
use gvar::Gvar;
use ident::Ident;
use numbers::parse_number;
use percent::parse_percent;
use strings::parse_string;

use strings::{action::StringExtendAction, literal::StringLiteral};

#[derive(Debug)]
pub struct Lexer {
    debug: bool,
    pub(crate) state: StateRef,
}

impl HasStateRef for Lexer {
    fn state_ref(&self) -> StateRef {
        self.state
    }
}
generate_state_ref_delegation!(Lexer);
impl Lexer {
    pub(crate) fn skip_byte(&mut self) {
        self.buffer().skip_byte()
    }
    pub(crate) fn current_byte(&self) -> Option<u8> {
        self.buffer().current_byte()
    }
    pub(crate) fn pos(&self) -> usize {
        self.buffer().pos()
    }
}

impl Lexer {
    pub(crate) fn new(state_ref: StateRef) -> Self {
        Self {
            debug: false,
            state: state_ref,
        }
    }

    #[cfg(test)]
    pub(crate) fn new_managed(input: &[u8]) -> (Self, OwnedState) {
        let mut state = OwnedState::new(input);
        let state_ref = state.new_ref();
        (Self::new(state_ref), state)
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    pub fn current_token(&mut self) -> Token {
        if let Some(current_token) = self.tokens().get(self.token_idx()) {
            *current_token
        } else {
            // get new token
            let token = self.next_token();
            self.tokens_mut().push(token);
            token
        }
    }

    fn next_token(&mut self) -> Token {
        let token = if self.string_literals().last().is_some() {
            self.tokenize_while_in_string()
        } else {
            self.tokenize_normally()
        };
        if self.debug {
            println!("Returning token {:?}", token);
        }

        // Reset one-time flag
        *self.required_new_expr_mut() = false;

        token
    }

    pub(crate) fn lookahead_is_identifier(&self) -> bool {
        Ident::lookahead(self.buffer().for_lookahead(), self.buffer().pos()).is_some()
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
        *self.required_new_expr_mut() = true;
    }

    pub(crate) fn skip_token(&mut self) {
        self.state_ref()
            .set_token_idx(self.state_ref().token_idx() + 1);
    }

    fn tokenize_while_in_string(&mut self) -> Token {
        // SAFETY: this method is called only if `string_literals` has at least 1 item
        let literal = unsafe { self.string_literals().last_mut().unwrap_unchecked() };

        match parse_string(literal, self.buffer(), self.curly_nest()) {
            StringExtendAction::EmitToken { token } => {
                // just emit what literal gives us
                token
            }
            StringExtendAction::FoundStringEnd { token } => {
                // close current literal
                self.string_literals().pop();
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
                self.string_literals().pop();
                // and emit EOF
                token!(tEOF, loc!(eof_pos, eof_pos))
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
        if self.buffer().lookahead(b"TEST_TOKEN") {
            let end = start + "TEST_TOKEN".len();
            self.buffer().set_pos(end);
            return token!(tTEST_TOKEN, loc!(start, end));
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
                self.buffer().set_pos(start);
                parse_number(self.buffer())
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
                self.buffer().set_pos(start);
                let (literal, token) = parse_percent(self.buffer(), self.curly_nest());
                if let Some(literal) = literal {
                    self.string_literals().push(literal);
                }
                token
            }
            b'$' => {
                self.buffer().set_pos(start);
                Gvar::parse(self.buffer())
            }
            b'@' => {
                self.buffer().set_pos(start);
                AtMark::parse(self.buffer())
            }
            b'_' => OnByte::<b'_'>::on_byte(self),

            _ident_start => {
                self.buffer().set_pos(start);
                Ident::parse(self.buffer())
            }
        }
    }
}

pub(crate) trait OnByte<const BYTE: u8> {
    fn on_byte(&mut self) -> Token;
}
