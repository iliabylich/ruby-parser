use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::BufferWithCursor,
        numbers::{
            state::{float::*, try_sub_parser, Float, Imaginary, Rational, State},
            try_to_extend_with, ExtendNumber, Number,
        },
    },
    token::TokenValue,
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Integer;

impl ExtendNumber for Integer {
    fn extend(number: &mut Number, buffer: &mut BufferWithCursor) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(try_to_extend_with::dot_number_suffix, buffer, start, number) {
            number.state = State::Float(Float::WithDotNumber(FloatWithDotNumber));
            return ControlFlow::Continue(());
        }

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

impl<'a> Into<TokenValue<'a>> for Integer {
    fn into(self) -> TokenValue<'a> {
        TokenValue::tINTEGER
    }
}
