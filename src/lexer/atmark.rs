use crate::lexer::{
    assert_lex,
    buffer::{Buffer, Lookahead, LookaheadResult},
    ident::Ident,
};
use crate::token::{Loc, Token, TokenValue};

pub(crate) struct AtMark;

pub(crate) enum LookaheadAtMarkResult {
    Ok(Token),
    InvalidVarName(Token),
    EmptyVarName(Token),
}

impl<'a> Lookahead<'a> for AtMark {
    type Output = LookaheadAtMarkResult;

    fn lookahead(buffer: &Buffer<'a>, start: usize) -> Self::Output {
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
            Some(byte) if !Ident::is_identchar(byte) => {
                // ditto
                return empty_var_name();
            }
            Some(byte) if byte.is_ascii_digit() => {
                return invalid_var_name();
            }
            Some(_) => {
                // read while possible
                match Ident::lookahead(buffer, ident_start) {
                    LookaheadResult::Some { length } => {
                        let ident_end = ident_start + length;
                        let token = Token(token_value, Loc(start, ident_end));

                        return LookaheadAtMarkResult::Ok(token);
                    }
                    LookaheadResult::None => {
                        // something like "@\xFF"
                        return invalid_var_name();
                    }
                }
            }
        }
    }
}

impl AtMark {
    pub(crate) fn parse(buffer: &mut Buffer) -> Token {
        let token = match AtMark::lookahead(buffer, buffer.pos()) {
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
}

assert_lex!(test_tIVAR_valid, b"@ivar", tIVAR, b"@ivar", 0..5);
assert_lex!(test_tCVAR_valid, b"@@cvar", tCVAR, b"@@cvar", 0..6);

assert_lex!(test_tIVAR_no_id, b"@", tIVAR, b"@", 0..1);
assert_lex!(test_tCVAR_no_id, b"@@", tCVAR, b"@@", 0..2);

assert_lex!(test_tIVAR_invalid_id, b"@(", tIVAR, b"@", 0..1);
assert_lex!(test_tCVAR_invalid_id, b"@@(", tCVAR, b"@@", 0..2);
