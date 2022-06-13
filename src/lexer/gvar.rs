use crate::lexer::{
    assert_lex,
    buffer::{utf8::Utf8Char, Buffer, BufferWithCursor, Lookahead, LookaheadResult},
    ident::Ident,
};
use crate::token::{token, Token};

pub(crate) struct Gvar;

pub(crate) enum LookaheadGvarResult<'a> {
    Ok(Token<'a>),
    InvalidVarName(Token<'a>),
    EmptyVarName(Token<'a>),
}

impl<'a> Lookahead<'a> for Gvar {
    type Output = LookaheadGvarResult<'a>;

    fn lookahead(buffer: &Buffer<'a>, start: usize) -> Self::Output {
        let mut ident_start = start + 1;

        let empty_gvar_name = || LookaheadGvarResult::EmptyVarName(token!(tGVAR, start, start + 1));

        let invalid_gvar_name =
            |end: usize| LookaheadGvarResult::InvalidVarName(token!(tGVAR, start, end));

        match buffer.byte_at(start + 1) {
            Some(b'_') => {
                /* $_: last read line string */
                match buffer.byte_at(start + 2) {
                    Some(byte) if Ident::is_identchar(byte) => {
                        ident_start += 1;
                    }
                    _ => {
                        // emit $_
                        return LookaheadGvarResult::Ok(token!(tGVAR, start, start + 2));
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
                return LookaheadGvarResult::Ok(token!(tGVAR, start, start + 2));
            }

            Some(b'-') => {
                match buffer.utf8_char_at(start + 2) {
                    Utf8Char::Valid { length } => {
                        // $-<UTF-8 char>
                        let end = start + 2 + length;
                        return LookaheadGvarResult::Ok(token!(tGVAR, start, end));
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
                return LookaheadGvarResult::Ok(token!(tBACK_REF, start, start + 2));
            }

            Some(b'1'..=b'9') => {
                // $NNNN
                let mut end = start + 2;
                while buffer.byte_at(end).map(|byte| byte.is_ascii_digit()) == Some(true) {
                    end += 1;
                }
                return LookaheadGvarResult::Ok(token!(tNTH_REF, start, end));
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
            LookaheadResult::Some { length } => {
                let end = ident_start + length;
                LookaheadGvarResult::Ok(token!(tGVAR, start, end))
            }
            LookaheadResult::None => {
                // $ (or $_) followed by invalid byte sequence
                invalid_gvar_name(ident_start)
            }
        }
    }
}

impl Gvar {
    pub(crate) fn parse<'a>(buffer: &mut BufferWithCursor<'a>) -> Token<'a> {
        let token = match Gvar::lookahead(buffer.for_lookahead(), buffer.pos()) {
            LookaheadGvarResult::Ok(token) => token,
            LookaheadGvarResult::InvalidVarName(token) => {
                // TODO: report __invalid__ ivar/cvar name
                token
            }
            LookaheadGvarResult::EmptyVarName(token) => {
                // TODO: report __empty__ ivar/cvar name
                token
            }
        };

        buffer.set_pos(token.loc().end());
        token
    }
}

assert_lex!(test_tGVAR_underscore_digits, b"$_42", tGVAR, b"$_42", 0..4);
assert_lex!(
    test_tGVAR_underscore_ascii_id,
    b"$_foo",
    tGVAR,
    b"$_foo",
    0..5
);
assert_lex!(
    test_tGVAR_underscore_utf8_id,
    // $_абв
    b"$_\xD0\xB0\xD0\xB1\xD0\xB2",
    tGVAR,
    b"$_\xD0\xB0\xD0\xB1\xD0\xB2",
    0..8 // foo
);
assert_lex!(
    test_tGVAR_underscore_invalid_bytes,
    &[b'$', b'_', 255],
    tGVAR,
    b"$_",
    0..2
);

// Special gvars
assert_lex!(test_tGVAR_match_data, b"$~", tGVAR, b"$~", 0..2);
assert_lex!(test_tGVAR_argv, b"$*", tGVAR, b"$*", 0..2);
assert_lex!(test_tGVAR_pid, b"$$", tGVAR, b"$$", 0..2);
assert_lex!(test_tGVAR_last_status, b"$?", tGVAR, b"$?", 0..2);
assert_lex!(test_tGVAR_error_string, b"$!", tGVAR, b"$!", 0..2);
assert_lex!(test_tGVAR_error_position, b"$@", tGVAR, b"$@", 0..2);
assert_lex!(test_tGVAR_input_record_separator, b"$/", tGVAR, b"$/", 0..2);
assert_lex!(
    test_tGVAR_output_record_separator,
    b"$\\",
    tGVAR,
    b"$\\",
    0..2
);
assert_lex!(test_tGVAR_field_separator, b"$;", tGVAR, b"$;", 0..2);
assert_lex!(test_tGVAR_output_field_separator, b"$,", tGVAR, b"$,", 0..2);
assert_lex!(test_tGVAR_last_read_line_number, b"$.", tGVAR, b"$.", 0..2);
assert_lex!(test_tGVAR_ignorecase, b"$=", tGVAR, b"$=", 0..2);
assert_lex!(test_tGVAR_load_path, b"$:", tGVAR, b"$:", 0..2);
assert_lex!(test_tGVAR_reading_filename, b"$<", tGVAR, b"$<", 0..2);
assert_lex!(test_tGVAR_default_output_handle, b"$>", tGVAR, b"$>", 0..2);
assert_lex!(test_tGVAR_already_loaded_files, b"$\"", tGVAR, b"$\"", 0..2);

// $-<identchar>
assert_lex!(test_tGVAR_dash_number, b"$-9", tGVAR, b"$-9", 0..3);
assert_lex!(test_tGVAR_dash_ascii, b"$-a", tGVAR, b"$-a", 0..3);
assert_lex!(
    test_tGVAR_dash_utf_8,
    b"$-\xD1\x84",
    tGVAR,
    b"$-\xD1\x84",
    0..4
);

// Special back refs
assert_lex!(test_tBACK_REF_last_match, b"$&", tBACK_REF, b"$&", 0..2);
assert_lex!(
    test_tBACK_REF_string_before_last_match,
    b"$`",
    tBACK_REF,
    b"$`",
    0..2
);
assert_lex!(
    test_tBACK_REF_string_after_last_match,
    b"$'",
    tBACK_REF,
    b"$'",
    0..2
);
assert_lex!(
    test_tBACK_REF_string_matches_last_paren,
    b"$+",
    tBACK_REF,
    b"$+",
    0..2
);

// $NNN
assert_lex!(test_tNTH_REF, b"$42", tNTH_REF, b"$42", 0..3);

assert_lex!(test_tGVAR_no_id, b"$ ", tGVAR, b"$", 0..1);
assert_lex!(test_tGVAR_invalid_id, b"$(", tGVAR, b"$", 0..1);

assert_lex!(test_tGVAR_ascii_id, b"$foo", tGVAR, b"$foo", 0..4);
assert_lex!(
    test_tGVAR_utf8_id,
    // $_абв
    b"$\xD0\xB0\xD0\xB1\xD0\xB2",
    tGVAR,
    b"$\xD0\xB0\xD0\xB1\xD0\xB2",
    0..7
);
assert_lex!(
    test_tGVAR_malformed_id,
    &[b'$', b'f', b'o', b'o', 208, 0],
    tGVAR,
    b"$foo",
    0..4
);
