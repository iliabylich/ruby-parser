use crate::buffer::Buffer;
use crate::lexer::Lexer;
use crate::nodes::Node;
use crate::state::OwnedState;
use crate::token::{Token, TokenKind};
pub(crate) use crate::transactions::{ParseError, ParseResult};

mod base;
use base::Rule;

mod alias;
mod arg;
mod args;
mod array;
mod case;
mod class;
mod def_method;
mod expr;
mod for_loop;
mod hash;
mod if_unless;
mod literal;
mod mlhs;
mod module;
mod pattern_matching;
mod postexe;
mod preexe;
mod primary;
mod rescue;
mod stmt;
mod trivial;
mod undef;
mod variables;

pub struct Parser {
    state: OwnedState,

    lexer: Lexer,
    debug: bool,
}

static mut COUNTER: usize = 0;

impl Parser {
    pub fn new(input: &[u8]) -> Self {
        let mut state = OwnedState::new(input);
        let state_ref = state.new_ref();

        Self {
            state,
            lexer: Lexer::new(state_ref),
            debug: false,
        }
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self.lexer = self.lexer.debug();
        self
    }

    pub(crate) fn current_token(&mut self) -> Token {
        self.lexer.current_token()
    }
    pub(crate) fn skip_token(&mut self) {
        self.lexer.skip_token()
    }

    pub(crate) fn expect_token(&mut self, expected: TokenKind) -> ParseResult<Token> {
        let token = self.current_token();
        self.skip_token();

        if token.is(expected) {
            Ok(token)
        } else {
            Err(ParseError::TokenError {
                lookahead: false,
                expected,
                got: token.kind,
                loc: token.loc,
            })
        }
    }

    pub(crate) fn try_token(&mut self, expected: TokenKind) -> ParseResult<Token> {
        if self.current_token().is(expected) {
            let token = self.current_token();
            self.skip_token();
            Ok(token)
        } else {
            Err(ParseError::TokenError {
                lookahead: true,
                expected,
                got: self.current_token().kind,
                loc: self.current_token().loc,
            })
        }
    }

    pub(crate) fn take_token(&mut self) -> Token {
        let token = self.current_token();
        self.skip_token();
        token
    }

    pub fn parse(&mut self) -> Option<Box<Node>> {
        Some(self.parse_program())
    }

    fn parse_program(&mut self) -> Box<Node> {
        stmt::TopCompstmt::parse(self).unwrap()
    }

    pub(crate) fn buffer(&self) -> &Buffer {
        self.lexer.buffer().for_lookahead()
    }
}
