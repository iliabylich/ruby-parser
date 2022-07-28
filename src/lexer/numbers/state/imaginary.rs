use std::ops::ControlFlow;

use crate::{
    buffer::BufferWithCursor,
    lexer::numbers::{ExtendNumber, Number},
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

impl Into<TokenKind> for Imaginary {
    fn into(self) -> TokenKind {
        TokenKind::tIMAGINARY
    }
}
