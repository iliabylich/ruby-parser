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

impl<'a> Lookahead<'a> for QMark {
    type Output = Token<'a>;

    fn lookahead(buffer: &Buffer<'a>, start: usize) -> Self::Output {
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
                            return token!(
                                tCHAR(char::from_u32(codepoint).unwrap()),
                                start,
                                start + length
                            );
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
                let codepoint = std::str::from_utf8(buffer.slice(start + 1, end))
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap();
                token!(tCHAR(codepoint), start, end)
            }
            _ => {
                token!(tEH, start, start + 1)
            }
        }
    }
}

impl QMark {
    pub(crate) fn parse<'a>(buffer: &mut Buffer<'a>) -> Token<'a> {
        let token = Self::lookahead(buffer, buffer.pos());
        buffer.set_pos(token.loc().end());
        token
    }
}

assert_lex!(test_tEH, b"?", tEH, b"?", 0..1);
assert_lex!(test_tCHAR_ascii, b"?a", tCHAR('a'), b"?a", 0..2);
assert_lex!(
    test_tCHAR_multibyte,
    "?字".as_bytes(),
    tCHAR('字'),
    "?字".as_bytes(),
    0..4
);
assert_lex!(test_tEH_and_ident, b"?ident", tEH, b"?", 0..1);
