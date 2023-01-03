use crate::buffer::Buffer;
use crate::lexer::Lexer;
use crate::nodes::Node;
use crate::token::{Token, TokenKind};

mod alias;
pub(crate) use alias::Alias;

mod args;
pub(crate) use args::{Args, CallArgs, Mrhs, OptParenArgs, ParenArgs};

mod array;
pub(crate) use array::Array;

mod base;

mod block;
pub(crate) use block::MaybeBlock;

mod case;
pub(crate) use case::Case;

mod def_method;
pub(crate) use def_method::{EndlessMethodDef, MethodDef};

mod def_module_class;
pub(crate) use def_module_class::{Class, Module};

mod for_loop;
pub(crate) use for_loop::ForLoop;

mod hash;
pub(crate) use hash::{Assoc, Hash};

mod if_unless;
pub(crate) use if_unless::{IfStmt, OptElse, Then, UnlessStmt};

mod keyword_cmd;
pub(crate) use keyword_cmd::KeywordCmd;

mod lambda;
pub(crate) use lambda::Lambda;

mod literal;
pub(crate) use literal::{Literal, StringContents, Symbol};

mod mlhs;
pub(crate) use mlhs::MLHS;

mod params;
pub(crate) use params::Params;

mod pattern_matching;
// pub(crate) use pattern_matching::{PCaseBody, PTopExprBody};

mod postexe;
pub(crate) use postexe::Postexe;

mod preexe;
pub(crate) use preexe::Preexe;

mod program;
pub(crate) use program::Program;

mod rescue;
pub(crate) use rescue::OptRescue;

mod stmt;
pub(crate) use stmt::{Bodystmt, Compstmt, OptTerms, TopStmts};

mod trivial;
pub(crate) use trivial::{
    BackRef, CnameT, Cvar, DoT, DotOrColon2T, DotT, FnameT, Gvar, Ivar, KeywordVariable, OpT,
    OperationT, SimpleNumeric, TermT, VarRef,
};

mod undef;
pub(crate) use undef::{Fitem, Undef};

mod value;
pub(crate) use value::Value;

pub struct Parser {
    lexer: Lexer,
    debug: bool,
}

impl Parser {
    pub fn new(input: &[u8]) -> Self {
        Self {
            lexer: Lexer::new(input),
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

    pub(crate) fn expect_token(&mut self, expected: TokenKind) -> Token {
        let token = self.current_token();
        self.skip_token();

        if token.is(expected) {
            token
        } else {
            panic!("Expected token {:?}, got {:?}", expected, token)
        }
    }

    pub(crate) fn take_token(&mut self) -> Token {
        let token = self.current_token();
        self.skip_token();
        token
    }

    pub fn parse(&mut self) -> Option<Box<Node>> {
        use base::Rule;
        Program::parse(self)
    }

    pub(crate) fn buffer(&self) -> &Buffer {
        self.lexer.buffer.for_lookahead()
    }
}
