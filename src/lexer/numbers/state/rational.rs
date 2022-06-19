use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::BufferWithCursor,
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
    fn extend(number: &mut Number, buffer: &mut BufferWithCursor) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(try_to_extend_with::i_suffix, buffer, start, number) {
            number.state = State::Imaginary(Imaginary);
            return ControlFlow::Continue(());
        }

        ControlFlow::Break(())
    }
}

impl<'a> Into<TokenValue<'a>> for Rational {
    fn into(self) -> TokenValue<'a> {
        TokenValue::tRATIONAL
    }
}