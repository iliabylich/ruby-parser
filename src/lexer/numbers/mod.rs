use std::ops::ControlFlow;

use crate::{
    lexer::buffer::{Buffer, BufferWithCursor},
    loc::loc,
    token::{Token, TokenKind},
};

pub(crate) mod scan;
pub(crate) mod try_to_extend_with;

mod state;
use state::{Float, Imaginary, Integer, IntegerPrefix, Rational, State, Uninitialized};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Number {
    state: State,
    begin: usize,
    end: usize,
}

impl Number {
    fn new(start: usize) -> Self {
        Self {
            state: State::Uninitialized(Uninitialized),
            begin: start,
            end: start,
        }
    }
}

trait ExtendNumber {
    fn extend(number: &mut Number, buffer: &mut BufferWithCursor) -> ControlFlow<()>;
}

impl ExtendNumber for Number {
    fn extend(number: &mut Number, buffer: &mut BufferWithCursor) -> ControlFlow<()> {
        match number.state {
            State::Uninitialized(_) => Uninitialized::extend(number, buffer),
            State::IntegerPrefix(_) => IntegerPrefix::extend(number, buffer),
            State::Integer(_) => Integer::extend(number, buffer),
            State::Rational(_) => Rational::extend(number, buffer),
            State::Imaginary(_) => Imaginary::extend(number, buffer),
            State::Float(_) => Float::extend(number, buffer),
        }
    }
}

impl<'a> Into<TokenKind<'a>> for Number {
    fn into(self) -> TokenKind<'a> {
        match self.state {
            State::Uninitialized(inner) => inner.into(),
            State::IntegerPrefix(inner) => inner.into(),
            State::Integer(inner) => inner.into(),
            State::Rational(inner) => inner.into(),
            State::Imaginary(inner) => inner.into(),
            State::Float(inner) => inner.into(),
        }
    }
}

impl<'a> Into<Token<'a>> for Number {
    fn into(self) -> Token<'a> {
        Token {
            kind: self.into(),
            loc: loc!(self.begin, self.end),
        }
    }
}

pub(crate) fn parse_number<'a>(buffer: &mut BufferWithCursor<'a>) -> Token<'a> {
    let mut number = Number::new(buffer.pos());

    loop {
        let action = Number::extend(&mut number, buffer);

        if action == ControlFlow::Break(()) {
            break;
        }
    }

    let token = number.into();
    println!("{:?}", token);
    token
}

#[cfg(test)]
mod tests;
