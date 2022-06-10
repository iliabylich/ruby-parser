use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::Buffer,
        numbers::{
            state::{try_sub_parser, Imaginary, Rational, State},
            try_to_extend_with, ExtendNumber, Number,
        },
    },
    token::TokenValue,
};

#[derive(Clone, Copy, Debug)]
pub(crate) enum Float {
    WithDotNumber(FloatWithDotNumber),
    WithESuffix(FloatWithESuffix),
}

impl ExtendNumber for Float {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        if let State::Float(float) = number.state {
            match float {
                Float::WithDotNumber(_) => FloatWithDotNumber::extend(number, buffer),
                Float::WithESuffix(_) => FloatWithESuffix::extend(number, buffer),
            }
        } else {
            unreachable!("bug")
        }
    }
}

impl<'a> Into<TokenValue<'a>> for Float {
    fn into(self) -> TokenValue<'a> {
        TokenValue::tFLOAT
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct FloatWithDotNumber;

impl ExtendNumber for FloatWithDotNumber {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(try_to_extend_with::e_suffix, buffer, start, number) {
            number.state = State::Float(Float::WithESuffix(FloatWithESuffix));
            return ControlFlow::Continue(());
        }

        if try_sub_parser!(try_to_extend_with::r_suffix, buffer, start, number) {
            number.state = State::Rational(Rational);
            return ControlFlow::Continue(());
        }

        if try_sub_parser!(try_to_extend_with::i_suffix, buffer, start, number) {
            number.state = State::Imaginary(Imaginary);
            return ControlFlow::Continue(());
        }

        ControlFlow::Break(())
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct FloatWithESuffix;

impl ExtendNumber for FloatWithESuffix {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(try_to_extend_with::r_suffix, buffer, start, number) {
            number.state = State::Rational(Rational);
            return ControlFlow::Continue(());
        }

        if try_sub_parser!(try_to_extend_with::i_suffix, buffer, start, number) {
            number.state = State::Imaginary(Imaginary);
            return ControlFlow::Continue(());
        }

        ControlFlow::Break(())
    }
}
