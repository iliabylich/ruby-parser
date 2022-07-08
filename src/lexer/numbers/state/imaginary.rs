use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::BufferWithCursor,
        numbers::{ExtendNumber, Number},
    },
    token::TokenKind,
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Imaginary;

impl ExtendNumber for Imaginary {
    fn extend(_number: &mut Number, _buffer: &mut BufferWithCursor) -> ControlFlow<()> {
        // Imaginary numbers can't be extended to anything bigger
        ControlFlow::Break(())
    }
}

impl<'a> Into<TokenKind<'a>> for Imaginary {
    fn into(self) -> TokenKind<'a> {
        TokenKind::tIMAGINARY
    }
}
