use crate::{
    lexer::{
        assert_lex,
        buffer::{utf8::Utf8Char, Buffer, BufferWithCursor, Lookahead, LookaheadResult},
        ident::Ident,
        strings::escapes::{
            Escape, EscapeError, SlashByte, SlashByteError, SlashMetaCtrl, SlashMetaCtrlError,
            SlashOctal, SlashOctalError, SlashU, SlashUError, SlashX, SlashXError,
        },
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
                    match Escape::lookahead(buffer, start + 1) {
                        Ok(None) => {
                            // no match
                        }
                        Ok(Some(escape)) => match escape {
                            // single char `?f` syntax doesn't support wide \u escapes
                            // because they may have multiple codepoints
                            Escape::SlashU(SlashU::Wide { codepoints, length }) => {
                                panic!(
                                    "wide codepoint in ?\\u syntax: {:?}, {}",
                                    codepoints, length
                                );
                            }

                            Escape::SlashU(SlashU::Short { codepoint, length }) => {
                                return token!(tCHAR(codepoint), start, start + length);
                            }

                            Escape::SlashOctal(SlashOctal { codepoint, length })
                            | Escape::SlashX(SlashX { codepoint, length })
                            | Escape::SlashMetaCtrl(SlashMetaCtrl { codepoint, length })
                            | Escape::SlashByte(SlashByte { codepoint, length }) => {
                                return token!(tCHAR(codepoint as char), start, start + length);
                            }
                        },
                        Err(err) => match err {
                            EscapeError::SlashUError(SlashUError {
                                codepoints,
                                errors,
                                length,
                            }) => {
                                panic!(
                                    "SlashUError {:?} / {:?} / {:?}",
                                    codepoints, errors, length
                                );
                            }
                            EscapeError::SlashOctalError(SlashOctalError { length }) => {
                                panic!("SlashOctalError {:?}", length);
                            }
                            EscapeError::SlashXError(SlashXError { length }) => {
                                panic!("SlashXError {:?}", length);
                            }
                            EscapeError::SlashMetaCtrlError(SlashMetaCtrlError { length }) => {
                                panic!("SlashMetaCtrlError {:?}", length)
                            }
                            EscapeError::SlashByteError(SlashByteError { length }) => {
                                panic!("SlashByteError {:?}", length)
                            }
                        },
                    }
                }
            }
            _ => {}
        }

        // just a ?C scharacter syntax
        match buffer.utf8_char_at(start + 1) {
            Utf8Char::Valid { length } => {
                let end = start + 1 + length;
                let codepoint = std::str::from_utf8(buffer.slice(start + 1, end).expect("bug"))
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
    pub(crate) fn parse<'a>(buffer: &mut BufferWithCursor<'a>) -> Token<'a> {
        let token = Self::lookahead(buffer.for_lookahead(), buffer.pos());
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
