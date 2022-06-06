use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::Buffer,
        numbers::{
            state::{try_sub_parser, Imaginary, State},
            try_to_extend_with, ExtendNumber, Number,
        },
    },
    token::TokenValue,
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Rational;

impl ExtendNumber for Rational {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(try_to_extend_with::i_suffix, buffer, start, number) {
            number.state = State::Imaginary(Imaginary);
            return ControlFlow::Continue(());
        }

        ControlFlow::Break(())
    }
}

impl Into<TokenValue> for Rational {
    fn into(self) -> TokenValue {
        TokenValue::tRATIONAL
    }
}
