use std::ops::ControlFlow;

use crate::{
    buffer::{Buffer, BufferWithCursor},
    loc::loc,
    token::{token, Token, TokenKind},
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

impl Into<TokenKind> for Number {
    fn into(self) -> TokenKind {
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

impl Into<Token> for Number {
    fn into(self) -> Token {
        token!(self.into(), loc!(self.begin, self.end))
    }
}

pub(crate) fn parse_number(buffer: &mut BufferWithCursor) -> Token {
    let mut number = Number::new(buffer.pos());

    loop {
        let action = Number::extend(&mut number, buffer);

        if action == ControlFlow::Break(()) {
            break;
        }
    }

    number.into()
}

#[cfg(test)]
mod tests;
