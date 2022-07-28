use crate::{
    lexer::{
        buffer::{utf8::Utf8Char, Buffer, BufferWithCursor},
        ident::Ident,
    },
    loc::loc,
    token::{token, Token},
};

pub(crate) struct Gvar {
    pub(crate) token: Token,
}

pub(crate) enum GvarError {
    InvalidVarName(Token),
    EmptyVarName(Token),
}

impl Gvar {
    pub(crate) fn lookahead(buffer: &Buffer, start: usize) -> Result<Gvar, GvarError> {
        let mut ident_start = start + 1;

        let empty_gvar_name = || {
            Err(GvarError::EmptyVarName(token!(
                tGVAR,
                loc!(start, start + 1)
            )))
        };

        let invalid_gvar_name =
            |end: usize| Err(GvarError::InvalidVarName(token!(tGVAR, loc!(start, end))));

        match buffer.byte_at(start + 1) {
            Some(b'_') => {
                /* $_: last read line string */
                match buffer.byte_at(start + 2) {
                    Some(byte) if Ident::is_identchar(byte) => {
                        ident_start += 1;
                    }
                    _ => {
                        // emit $_
                        return Ok(Gvar {
                            token: token!(tGVAR, loc!(start, start + 2)),
                        });
                    }
                }
            }

            // $~: match-data
            // $*: argv
            // $$: pid
            // $?: last status
            // $!: error string
            // $@: error position
            // $/: input record separator
            // $\: output record separator
            // $;: field separator
            // $,: output field separator
            // $.: last read line number
            // $=: ignorecase
            // $:: load path
            // $<: reading filename
            // $>: default output handle
            // $": already loaded files
            Some(b'~') | Some(b'*') | Some(b'$') | Some(b'?') | Some(b'!') | Some(b'@')
            | Some(b'/') | Some(b'\\') | Some(b';') | Some(b',') | Some(b'.') | Some(b'=')
            | Some(b':') | Some(b'<') | Some(b'>') | Some(b'\"') => {
                return Ok(Gvar {
                    token: token!(tGVAR, loc!(start, start + 2)),
                });
            }

            Some(b'-') => {
                match buffer.utf8_char_at(start + 2) {
                    Utf8Char::Valid { length } => {
                        // $-<UTF-8 char>
                        let end = start + 2 + length;
                        return Ok(Gvar {
                            token: token!(tGVAR, loc!(start, end)),
                        });
                    }
                    _ => {
                        // return just $-
                        return invalid_gvar_name(start + 2);
                    }
                }
            }

            // $&: last match
            // $`: string before last match
            // $': string after last match
            // $+: string matches last paren
            Some(b'&') | Some(b'`') | Some(b'\'') | Some(b'+') => {
                return Ok(Gvar {
                    token: token!(tBACK_REF, loc!(start, start + 2)),
                });
            }

            Some(b'1'..=b'9') => {
                // $NNNN
                let mut end = start + 2;
                while buffer.byte_at(end).map(|byte| byte.is_ascii_digit()) == Some(true) {
                    end += 1;
                }
                return Ok(Gvar {
                    token: token!(tNTH_REF, loc!(start, end)),
                });
            }

            Some(b' ') | None => {
                // Emit just `$`
                return empty_gvar_name();
            }

            _ => {
                // $<ident>
            }
        }

        match Ident::lookahead(buffer, ident_start) {
            Some(Ident { length }) => {
                let end = ident_start + length;
                Ok(Gvar {
                    token: token!(tGVAR, loc!(start, end)),
                })
            }
            None => {
                // $ (or $_) followed by invalid byte sequence
                invalid_gvar_name(ident_start)
            }
        }
    }
}

impl Gvar {
    pub(crate) fn parse(buffer: &mut BufferWithCursor) -> Token {
        let start = buffer.pos();
        let token = match Gvar::lookahead(buffer.for_lookahead(), start) {
            Ok(Gvar { token }) => token,
            Err(GvarError::InvalidVarName(token)) => {
                // TODO: report __invalid__ ivar/cvar name
                token
            }
            Err(GvarError::EmptyVarName(token)) => {
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
    fn test_tGVAR_underscore_digits() {
        assert_lex!(b"$_42", token!(tGVAR, loc!(0, 4)));
    }
    #[test]
    fn test_tGVAR_underscore_ascii_id() {
        assert_lex!(b"$_foo", token!(tGVAR, loc!(0, 5)));
    }
    #[test]
    fn test_tGVAR_underscore_utf8_id() {
        assert_lex!(
            // $_абв
            b"$_\xD0\xB0\xD0\xB1\xD0\xB2",
            token!(tGVAR, loc!(0, 8)) // foo
        );
    }
    #[test]
    fn test_tGVAR_underscore_invalid_bytes() {
        assert_lex!(&[b'$', b'_', 255], token!(tGVAR, loc!(0, 2)));
    }

    // Special gvars
    #[test]
    fn test_tGVAR_match_data() {
        assert_lex!(b"$~", token!(tGVAR, loc!(0, 2)));
    }
    #[test]
    fn test_tGVAR_argv() {
        assert_lex!(b"$*", token!(tGVAR, loc!(0, 2)));
    }
    #[test]
    fn test_tGVAR_pid() {
        assert_lex!(b"$$", token!(tGVAR, loc!(0, 2)));
    }
    #[test]
    fn test_tGVAR_last_status() {
        assert_lex!(b"$?", token!(tGVAR, loc!(0, 2)));
    }
    #[test]
    fn test_tGVAR_error_string() {
        assert_lex!(b"$!", token!(tGVAR, loc!(0, 2)));
    }
    #[test]
    fn test_tGVAR_error_position() {
        assert_lex!(b"$@", token!(tGVAR, loc!(0, 2)));
    }
    #[test]
    fn test_tGVAR_input_record_separator() {
        assert_lex!(b"$/", token!(tGVAR, loc!(0, 2)));
    }
    #[test]
    fn test_tGVAR_output_record_separator() {
        assert_lex!(b"$\\", token!(tGVAR, loc!(0, 2)));
    }
    #[test]
    fn test_tGVAR_field_separator() {
        assert_lex!(b"$;", token!(tGVAR, loc!(0, 2)));
    }
    #[test]
    fn test_tGVAR_output_field_separator() {
        assert_lex!(b"$,", token!(tGVAR, loc!(0, 2)));
    }
    #[test]
    fn test_tGVAR_last_read_line_number() {
        assert_lex!(b"$.", token!(tGVAR, loc!(0, 2)));
    }
    #[test]
    fn test_tGVAR_ignorecase() {
        assert_lex!(b"$=", token!(tGVAR, loc!(0, 2)));
    }
    #[test]
    fn test_tGVAR_load_path() {
        assert_lex!(b"$:", token!(tGVAR, loc!(0, 2)));
    }
    #[test]
    fn test_tGVAR_reading_filename() {
        assert_lex!(b"$<", token!(tGVAR, loc!(0, 2)));
    }
    #[test]
    fn test_tGVAR_default_output_handle() {
        assert_lex!(b"$>", token!(tGVAR, loc!(0, 2)));
    }
    #[test]
    fn test_tGVAR_already_loaded_files() {
        assert_lex!(b"$\"", token!(tGVAR, loc!(0, 2)));
    }

    // $-<identchar>
    #[test]
    fn test_tGVAR_dash_number() {
        assert_lex!(b"$-9", token!(tGVAR, loc!(0, 3)));
    }
    #[test]
    fn test_tGVAR_dash_ascii() {
        assert_lex!(b"$-a", token!(tGVAR, loc!(0, 3)));
    }
    #[test]
    fn test_tGVAR_dash_utf_8() {
        assert_lex!(b"$-\xD1\x84", token!(tGVAR, loc!(0, 4)));
    }

    // Special back refs
    #[test]
    fn test_tBACK_REF_last_match() {
        assert_lex!(b"$&", token!(tBACK_REF, loc!(0, 2)));
    }
    #[test]
    fn test_tBACK_REF_string_before_last_match() {
        assert_lex!(b"$`", token!(tBACK_REF, loc!(0, 2)));
    }
    #[test]
    fn test_tBACK_REF_string_after_last_match() {
        assert_lex!(b"$'", token!(tBACK_REF, loc!(0, 2)));
    }
    #[test]
    fn test_tBACK_REF_string_matches_last_paren() {
        assert_lex!(b"$+", token!(tBACK_REF, loc!(0, 2)));
    }

    // $NNN
    #[test]
    fn test_tNTH_REF() {
        assert_lex!(b"$42", token!(tNTH_REF, loc!(0, 3)));
    }

    #[test]
    fn test_tGVAR_no_id() {
        assert_lex!(b"$ ", token!(tGVAR, loc!(0, 1)));
    }
    #[test]
    fn test_tGVAR_invalid_id() {
        assert_lex!(b"$(", token!(tGVAR, loc!(0, 1)));
    }

    #[test]
    fn test_tGVAR_ascii_id() {
        assert_lex!(b"$foo", token!(tGVAR, loc!(0, 4)));
    }
    #[test]
    fn test_tGVAR_utf8_id() {
        assert_lex!(
            // $_абв
            b"$\xD0\xB0\xD0\xB1\xD0\xB2",
            token!(tGVAR, loc!(0, 7))
        );
    }
    #[test]
    fn test_tGVAR_malformed_id() {
        assert_lex!(&[b'$', b'f', b'o', b'o', 208, 0], token!(tGVAR, loc!(0, 4)));
    }
}
