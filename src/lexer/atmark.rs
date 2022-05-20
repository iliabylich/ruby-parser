use crate::lexer::buffer::Buffer;
use crate::token::{Loc, Token, TokenValue};

use crate::lexer::ident::is_identchar;

pub(crate) fn parse_atmark<'a>(buffer: &mut Buffer<'a>) -> Token<'a> {
    let start = buffer.pos();
    buffer.skip_byte();

    let mut token_value_fn: fn(&'a [u8]) -> TokenValue<'a> = TokenValue::tIVAR;

    match buffer.current_byte() {
        Some(b'@') => {
            // @@
            buffer.skip_byte();
            token_value_fn = TokenValue::tCVAR;
        }
        _ => {}
    }

    let ident_start = buffer.pos();

    while buffer.current_byte().map(|byte| is_identchar(byte)) == Some(true) {
        buffer.skip_byte();
    }

    let end = buffer.pos();
    if ident_start == end {
        match buffer.byte_at(ident_start) {
            Some(b'0'..=b'9') => {
                // TODO: report __invalid__ ivar/cvar name
            }
            None | Some(_) => {
                // TODO: report __empty__ ivar/cvar name
            }
        }
    }
    Token(token_value_fn(buffer.slice(start, end)), Loc(start, end))
}

#[cfg(test)]
mod tests {
    use crate::lexer::assert_lex;

    assert_lex!(test_tIVAR_valid, "@ivar", tIVAR(b"@ivar"), 0..5);
    assert_lex!(test_tCVAR_valid, "@@cvar", tCVAR(b"@@cvar"), 0..6);

    assert_lex!(test_tIVAR_no_id, "@", tIVAR(b"@"), 0..1);
    assert_lex!(test_tCVAR_no_id, "@@", tCVAR(b"@@"), 0..2);

    assert_lex!(test_tIVAR_invalid_id, "@(", tIVAR(b"@"), 0..1);
    assert_lex!(test_tCVAR_invalid_id, "@@(", tCVAR(b"@@"), 0..2);
}
