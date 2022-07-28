use crate::{
    lexer::{
        buffer::{Buffer, BufferWithCursor},
        ident::Ident,
    },
    loc::loc,
    token::{token, Token, TokenKind},
};

pub(crate) struct AtMark {
    pub(crate) token: Token,
}

pub(crate) enum AtMarkError {
    InvalidVarName(Token),
    EmptyVarName(Token),
}

impl AtMark {
    pub(crate) fn lookahead(buffer: &Buffer, start: usize) -> Result<AtMark, AtMarkError> {
        let mut ident_start = start + 1;

        let mut token_value = TokenKind::tIVAR;

        match buffer.byte_at(start + 1) {
            Some(b'@') => {
                // @@
                token_value = TokenKind::tCVAR;
                ident_start += 1;
            }
            _ => {}
        }

        let empty_var_name = |token_value: TokenKind| {
            Err(AtMarkError::EmptyVarName(token!(
                token_value,
                loc!(start, ident_start)
            )))
        };

        let invalid_var_name = |token_value: TokenKind| {
            Err(AtMarkError::InvalidVarName(token!(
                token_value,
                loc!(start, ident_start)
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
                        let token = token!(token_value, loc!(start, ident_end));

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

impl AtMark {
    pub(crate) fn parse(buffer: &mut BufferWithCursor) -> Token {
        let start = buffer.pos();
        let token = match AtMark::lookahead(buffer.for_lookahead(), start) {
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

        buffer.set_pos(token.loc.end);
        token
    }
}

#[cfg(test)]
mod tests {
    use crate::{testing::assert_lex, token::token};

    #[test]
    fn test_tIVAR_valid() {
        assert_lex!(b"@ivar", token!(tIVAR, loc!(0, 5)));
    }
    #[test]
    fn test_tCVAR_valid() {
        assert_lex!(b"@@cvar", token!(tCVAR, loc!(0, 6)));
    }

    #[test]
    fn test_tIVAR_no_id() {
        assert_lex!(b"@", token!(tIVAR, loc!(0, 1)));
    }
    #[test]
    fn test_tCVAR_no_id() {
        assert_lex!(b"@@", token!(tCVAR, loc!(0, 2)));
    }

    #[test]
    fn test_tIVAR_invalid_id() {
        assert_lex!(b"@(", token!(tIVAR, loc!(0, 1)));
    }
    #[test]
    fn test_tCVAR_invalid_id() {
        assert_lex!(b"@@(", token!(tCVAR, loc!(0, 2)));
    }
}
