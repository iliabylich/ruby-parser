use crate::{
    lexer::{
        assert_lex,
        buffer::{utf8::Utf8Char, Buffer, Lookahead, LookaheadResult},
        ident::Ident,
        strings::escapes::{LooakeadhSlashUResult, SlashU},
    },
    token::{token, Token},
};

pub(crate) struct QMark;

impl Lookahead for QMark {
    type Output = Token;

    fn lookahead(buffer: &Buffer, start: usize) -> Self::Output {
        match buffer.byte_at(start + 1) {
            Some(byte) => {
                if (byte.is_ascii_alphanumeric() || byte == b'_')
                    && matches!(
                        Ident::lookahead(buffer, start + 1),
                        LookaheadResult::Some { length: 2.. }
                    )
                {
                    // split ?ident into `?` + `ident`
                    // TODO: warn about ambiguity
                    return token!(tEH, start, start + 1);
                } else if byte == b'\\' {
                    match dbg!(SlashU::lookahead(buffer, start + 1)) {
                        LooakeadhSlashUResult::Short { codepoint, length } => {
                            return token!(tCHAR, start, start + length);
                        }
                        LooakeadhSlashUResult::Wide { codepoints, length } => {
                            panic!(
                                "wide codepoint in ?\\u syntax: {:?}, {}",
                                codepoints, length
                            );
                        }
                        LooakeadhSlashUResult::Err {
                            codepoints,
                            errors,
                            length,
                        } => {
                            panic!(
                            "got errors {:?} during parsing ?\\u syntax (codepoints = {:?}, length = {})",
                            errors,
                            codepoints,
                            length
                        );
                        }
                    }
                }
            }
            _ => {}
        }

        // just a ?C scharacter syntax
        match buffer.utf8_char_at(start + 1) {
            Utf8Char::Valid { length } => {
                let end = start + 1 + length;
                token!(tCHAR, start, end)
            }
            _ => {
                token!(tEH, start, start + 1)
            }
        }
    }
}

impl QMark {
    pub(crate) fn parse(buffer: &mut Buffer) -> Token {
        let token = Self::lookahead(buffer, buffer.pos());
        buffer.set_pos(token.loc().end());
        token
    }
}

assert_lex!(test_tEH, b"?", tEH, b"?", 0..1);
assert_lex!(test_tCHAR_ascii, b"?a", tCHAR, b"?a", 0..2);
assert_lex!(
    test_tCHAR_multibyte,
    "?字".as_bytes(),
    tCHAR,
    "?字".as_bytes(),
    0..4
);
assert_lex!(test_tEH_and_ident, b"?ident", tEH, b"?", 0..1);
