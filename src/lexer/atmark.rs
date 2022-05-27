use crate::lexer::buffer::Buffer;
use crate::token::{Loc, Token, TokenValue};

use crate::lexer::ident::is_identchar;

pub(crate) fn parse_atmark<'a>(buffer: &mut Buffer<'a>) -> Token<'a> {
    let token = match lookahead_atmark(buffer) {
        Ok(token) => token,
        Err(incomplete_token) => {
            match buffer.byte_at(incomplete_token.loc().end()) {
                Some(b'0'..=b'9') => {
                    // TODO: report __invalid__ ivar/cvar name
                }
                None | Some(_) => {
                    // TODO: report __empty__ ivar/cvar name
                }
            }
            incomplete_token
        }
    };

    buffer.set_pos(token.loc().end());
    token
}

// Returns Ok(Token) or Err(Token with only '@' / '@@')
pub(crate) fn lookahead_atmark<'a>(buffer: &Buffer<'a>) -> Result<Token<'a>, Token<'a>> {
    let start = buffer.pos();
    let mut ident_start = buffer.pos() + 1;

    let mut token_value_fn: fn(&'a [u8]) -> TokenValue<'a> = TokenValue::tIVAR;

    match buffer.byte_at(start + 1) {
        Some(b'@') => {
            // @@
            token_value_fn = TokenValue::tCVAR;
            ident_start += 1;
        }
        _ => {}
    }

    let mut ident_end = ident_start;
    while buffer.byte_at(ident_end).map(|byte| is_identchar(byte)) == Some(true) {
        ident_end += 1;
    }

    let token = Token(
        token_value_fn(buffer.slice(start, ident_end)),
        Loc(start, ident_end),
    );

    if ident_start == ident_end {
        Err(token)
    } else {
        Ok(token)
    }
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
