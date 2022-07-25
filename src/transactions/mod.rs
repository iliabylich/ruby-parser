use crate::{builder::Constructor, lexer::Checkpoint, parser::Parser, token::TokenKind};

mod error;
pub(crate) use error::{ParseError, StepData};

mod result;
pub(crate) use result::{ParseResult, ParseResultApi};

mod one_of;
use one_of::OneOf;

mod all_of;
use all_of::AllOf;

impl<C: Constructor> Parser<C> {
    pub(crate) fn one_of<T>(&self, name: &'static str) -> OneOf<T> {
        OneOf::new(name, self.new_checkpoint())
    }

    pub(crate) fn all_of(&self, name: &'static str) -> AllOf {
        AllOf::new(name)
    }
}

mod render;
#[cfg(test)]
pub(crate) use render::assert_err_eq;
