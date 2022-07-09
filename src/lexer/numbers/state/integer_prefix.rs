use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::BufferWithCursor,
        numbers::{
            scan,
            state::{try_sub_parser, Integer, State},
            ExtendNumber, Number,
        },
    },
    token::TokenKind,
};

#[derive(Clone, Copy, Debug)]
pub(crate) enum IntegerPrefix {
    Hexadecimal(Hexadecimal),
    Binary(Binary),
    Octal(Octal),
    Decimal(Decimal),
}

impl ExtendNumber for IntegerPrefix {
    fn extend(number: &mut Number, buffer: &mut BufferWithCursor) -> ControlFlow<()> {
        if let State::IntegerPrefix(prefix) = number.state {
            match prefix {
                IntegerPrefix::Hexadecimal(_) => Hexadecimal::extend(number, buffer),
                IntegerPrefix::Binary(_) => Binary::extend(number, buffer),
                IntegerPrefix::Octal(_) => Octal::extend(number, buffer),
                IntegerPrefix::Decimal(_) => Decimal::extend(number, buffer),
            }
        } else {
            unreachable!("bug")
        }
    }
}

impl Into<TokenKind> for IntegerPrefix {
    fn into(self) -> TokenKind {
        unreachable!("ExtendNumber made an incomplete transition to {:?}", self)
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Hexadecimal;

// Runs after consuming `0x` hexadecimal prefix
impl ExtendNumber for Hexadecimal {
    fn extend(number: &mut Number, buffer: &mut BufferWithCursor) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(scan::hexadecimal, buffer, start, number) {
            number.state = State::Integer(Integer);
            ControlFlow::Continue(())
        } else {
            panic!("numeric literal without digits")
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Binary;

// Runs after consuming `0b` binary prefix
impl ExtendNumber for Binary {
    fn extend(number: &mut Number, buffer: &mut BufferWithCursor) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(scan::binary, buffer, start, number) {
            number.state = State::Integer(Integer);
            ControlFlow::Continue(())
        } else {
            panic!("numeric literal without digits")
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Octal;

// Runs after consuming octal prefix (`0`)
impl ExtendNumber for Octal {
    fn extend(number: &mut Number, buffer: &mut BufferWithCursor) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(scan::octal, buffer, start, number) {
            number.state = State::Integer(Integer);
            ControlFlow::Continue(())
        } else {
            panic!("numeric literal without digits")
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Decimal;

// Runs after consuming decimal prefix (`0d` or no prefix)
impl ExtendNumber for Decimal {
    fn extend(number: &mut Number, buffer: &mut BufferWithCursor) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(scan::decimal, buffer, start, number) {
            number.state = State::Integer(Integer);
            return ControlFlow::Continue(());
        } else {
            panic!("numeric literal without digits")
        }
    }
}
