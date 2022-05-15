use crate::lexer::buffer::Buffer;
use crate::token::{Loc, Token, TokenValue};

pub(crate) fn parse_number<'a>(buffer: &mut Buffer<'a>) -> Token<'a> {
    let start = buffer.pos();

    // todo: parse numeric
    while let Some(b'0'..=b'9') = buffer.current_byte() {
        buffer.skip_byte();
    }
    let num = buffer.slice(start, buffer.pos());
    Token(TokenValue::tINTEGER(num), Loc(start, buffer.pos()))
}
