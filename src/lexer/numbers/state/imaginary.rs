use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::Buffer,
        numbers::{ExtendNumber, Number},
    },
    token::TokenValue,
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Imaginary;

impl ExtendNumber for Imaginary {
    fn extend(_number: &mut Number, _buffer: &mut Buffer) -> ControlFlow<()> {
        // Imaginary numbers can't be extended to anything bigger
        ControlFlow::Break(())
    }
}

impl Into<TokenValue> for Imaginary {
    fn into(self) -> TokenValue {
        TokenValue::tIMAGINARY
    }
}
