use crate::{builder::Constructor, lexer::Checkpoint, parser::Parser, token::TokenKind};

mod error;
pub(crate) use error::{Expectation, ParseError, ParseErrorDetails, StepError};

mod result;
pub(crate) use result::{ParseResult, ParseResultApi};

mod one_of;
use one_of::OneOf;

impl<C: Constructor> Parser<C> {
    pub(crate) fn one_of<T>(&self, name: &'static str) -> OneOf<T> {
        OneOf::new(name, self.new_checkpoint())
    }
}
