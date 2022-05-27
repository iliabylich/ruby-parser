use crate::lexer::buffer::Buffer;
use crate::token::{Loc, Token, TokenValue};

use crate::lexer::ident::is_identchar;

pub(crate) fn parse_atmark<'a>(buffer: &mut Buffer<'a>) -> Token<'a> {
    let token = match lookahead_atmark(buffer, buffer.pos()) {
        LookaheadAtMarkResult::Ok(token) => token,
        LookaheadAtMarkResult::InvalidVarName(token) => {
            // TODO: report __invalid__ ivar/cvar name
            token
        }
        LookaheadAtMarkResult::EmptyVarName(token) => {
            // TODO: report __empty__ ivar/cvar name
            token
        }
    };

    buffer.set_pos(token.loc().end());
    token
}

pub(crate) enum LookaheadAtMarkResult<'a> {
    Ok(Token<'a>),
    InvalidVarName(Token<'a>),
    EmptyVarName(Token<'a>),
}

// Returns Ok(Token) or Err(Token with only '@' / '@@')
pub(crate) fn lookahead_atmark<'a>(buffer: &Buffer<'a>, start: usize) -> LookaheadAtMarkResult<'a> {
    let mut ident_start = start + 1;

    let mut token_value_fn: fn(&'a [u8]) -> TokenValue<'a> = TokenValue::tIVAR;

    match buffer.byte_at(start + 1) {
        Some(b'@') => {
            // @@
            token_value_fn = TokenValue::tCVAR;
            ident_start += 1;
        }
        _ => {}
    }

    let empty_var_name = || {
        LookaheadAtMarkResult::EmptyVarName(Token(
            token_value_fn(buffer.slice(start, ident_start)),
            Loc(start, ident_start),
        ))
    };

    match buffer.byte_at(ident_start) {
        None => {
            return empty_var_name();
        }
        Some(byte) if !is_identchar(byte) => {
            // ditto
            return empty_var_name();
        }
        Some(byte) if byte.is_ascii_digit() => {
            return LookaheadAtMarkResult::InvalidVarName(Token(
                token_value_fn(buffer.slice(start, ident_start)),
                Loc(start, ident_start),
            ));
        }
        Some(_) => {
            // read while possible
            let mut ident_end = ident_start;
            while buffer.byte_at(ident_end).map(|byte| is_identchar(byte)) == Some(true) {
                ident_end += 1;
            }

            let token = Token(
                token_value_fn(buffer.slice(start, ident_end)),
                Loc(start, ident_end),
            );

            LookaheadAtMarkResult::Ok(token)
        }
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
