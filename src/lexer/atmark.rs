use crate::lexer::{
    assert_lex,
    buffer::Buffer,
    ident::{is_identchar, lookahead_ident},
};
use crate::token::{Loc, Token, TokenValue};

pub(crate) fn parse_atmark<'a>(buffer: &mut Buffer<'a>) -> Token {
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

pub(crate) enum LookaheadAtMarkResult {
    Ok(Token),
    InvalidVarName(Token),
    EmptyVarName(Token),
}

// Returns Ok(Token) or Err(Token with only '@' / '@@')
pub(crate) fn lookahead_atmark<'a>(buffer: &Buffer<'a>, start: usize) -> LookaheadAtMarkResult {
    let mut ident_start = start + 1;

    let mut token_value = TokenValue::tIVAR;

    match buffer.byte_at(start + 1) {
        Some(b'@') => {
            // @@
            token_value = TokenValue::tCVAR;
            ident_start += 1;
        }
        _ => {}
    }

    let empty_var_name =
        || LookaheadAtMarkResult::EmptyVarName(Token(token_value, Loc(start, ident_start)));

    let invalid_var_name =
        || LookaheadAtMarkResult::InvalidVarName(Token(token_value, Loc(start, ident_start)));

    match buffer.byte_at(ident_start) {
        None => {
            return empty_var_name();
        }
        Some(byte) if !is_identchar(byte) => {
            // ditto
            return empty_var_name();
        }
        Some(byte) if byte.is_ascii_digit() => {
            return invalid_var_name();
        }
        Some(_) => {
            // read while possible
            match lookahead_ident(buffer, ident_start) {
                Some(length) => {
                    let ident_end = ident_start + length;
                    let token = Token(token_value, Loc(start, ident_end));

                    return LookaheadAtMarkResult::Ok(token);
                }
                None => {
                    // something like "@\xFF"
                    return invalid_var_name();
                }
            }
        }
    }
}

assert_lex!(test_tIVAR_valid, b"@ivar", tIVAR, b"@ivar", 0..5);
assert_lex!(test_tCVAR_valid, b"@@cvar", tCVAR, b"@@cvar", 0..6);

assert_lex!(test_tIVAR_no_id, b"@", tIVAR, b"@", 0..1);
assert_lex!(test_tCVAR_no_id, b"@@", tCVAR, b"@@", 0..2);

assert_lex!(test_tIVAR_invalid_id, b"@(", tIVAR, b"@", 0..1);
assert_lex!(test_tCVAR_invalid_id, b"@@(", tCVAR, b"@@", 0..2);
