use crate::{
    lexer::{
        assert_lex,
        buffer::{Buffer, BufferWithCursor, Lookahead},
        ident::Ident,
    },
    token::{Token, TokenValue},
    Loc,
};

pub(crate) struct AtMark<'a> {
    pub(crate) token: Token<'a>,
}

pub(crate) enum AtMarkError<'a> {
    InvalidVarName(Token<'a>),
    EmptyVarName(Token<'a>),
}

impl<'a> Lookahead<'a> for AtMark<'a> {
    type Output = Result<AtMark<'a>, AtMarkError<'a>>;

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

        let empty_var_name = |token_value: TokenValue<'a>| {
            Err(AtMarkError::EmptyVarName(Token(
                token_value,
                Loc {
                    start,
                    end: ident_start,
                },
            )))
        };

        let invalid_var_name = |token_value: TokenValue<'a>| {
            Err(AtMarkError::InvalidVarName(Token(
                token_value,
                Loc {
                    start,
                    end: ident_start,
                },
            )))
        };

        match buffer.byte_at(ident_start) {
            None => {
                return empty_var_name(token_value);
            }
            Some(byte) if !Ident::is_identchar(byte) => {
                // ditto
                return empty_var_name(token_value);
            }
            Some(byte) if byte.is_ascii_digit() => {
                return invalid_var_name(token_value);
            }
            Some(_) => {
                // read while possible
                match Ident::lookahead(buffer, ident_start) {
                    Some(Ident { length }) => {
                        let ident_end = ident_start + length;
                        let token = Token(
                            token_value,
                            Loc {
                                start,
                                end: ident_end,
                            },
                        );

                        return Ok(AtMark { token });
                    }
                    None => {
                        // something like "@\xFF"
                        return invalid_var_name(token_value);
                    }
                }
            }
        }
    }
}

impl<'a> AtMark<'a> {
    pub(crate) fn parse(buffer: &mut BufferWithCursor<'a>) -> Token<'a> {
        let token = match AtMark::lookahead(buffer.for_lookahead(), buffer.pos()) {
            Ok(AtMark { token }) => token,
            Err(AtMarkError::InvalidVarName(token)) => {
                // TODO: report __invalid__ ivar/cvar name
                token
            }
            Err(AtMarkError::EmptyVarName(token)) => {
                // TODO: report __empty__ ivar/cvar name
                token
            }
        };

        buffer.set_pos(token.loc().end);
        token
    }
}

assert_lex!(test_tIVAR_valid, b"@ivar", tIVAR, b"@ivar", 0..5);
assert_lex!(test_tCVAR_valid, b"@@cvar", tCVAR, b"@@cvar", 0..6);

assert_lex!(test_tIVAR_no_id, b"@", tIVAR, b"@", 0..1);
assert_lex!(test_tCVAR_no_id, b"@@", tCVAR, b"@@", 0..2);

assert_lex!(test_tIVAR_invalid_id, b"@(", tIVAR, b"@", 0..1);
assert_lex!(test_tCVAR_invalid_id, b"@@(", tCVAR, b"@@", 0..2);
