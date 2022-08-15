use crate::parser::Parser;

mod step_data;

mod steps;

mod error;
pub(crate) use error::ParseError;

mod result;
pub(crate) use result::ParseResult;

mod one_of;
use one_of::OneOf;

impl Parser {
    pub(crate) fn one_of<T>(&self, name: &'static str) -> OneOf<T> {
        // eprintln!("constructing one_of {:?}", name);
        OneOf::new(name, self.new_checkpoint())
    }
}

#[cfg(test)]
mod render;
