use crate::{
    buffer::{utf8::Utf8Char, Buffer, BufferWithCursor},
    lexer::{
        ident::Ident,
        strings::escapes::{
            Escape, EscapeError, SlashByte, SlashByteError, SlashMetaCtrl, SlashMetaCtrlError,
            SlashOctal, SlashU, SlashUError, SlashX, SlashXError,
        },
    },
    loc::loc,
    token::{token, Token},
};

pub(crate) struct QMark {
    token: Token,
}

impl QMark {
    pub(crate) fn lookahead(buffer: &mut Buffer, start: usize) -> Self {
        match buffer.byte_at(start + 1) {
            Some(byte) => {
                if (byte.is_ascii_alphanumeric() || byte == b'_')
                    && matches!(
                        Ident::lookahead(buffer, start + 1),
                        Some(Ident { length: 2.. })
                    )
                {
                    // split ?ident into `?` + `ident`
                    // TODO: warn about ambiguity
                    return QMark {
                        token: token!(tEH, loc!(start, start + 1)),
                    };
                } else if byte == b'\\' {
                    match Escape::lookahead(buffer, start + 1) {
                        Ok(None) => {
                            // no match
                        }
                        Ok(Some(escape)) => match escape {
                            // single char `?f` syntax doesn't support wide \u escapes
                            // because they may have multiple codepoints
                            Escape::SlashU(SlashU::Wide {
                                escaped_loc,
                                length,
                            }) => {
                                panic!(
                                    "wide codepoint in ?\\u syntax: {:?}, {}",
                                    escaped_loc, length
                                );
                            }

                            Escape::SlashU(SlashU::Short {
                                codepoint: bytes,
                                length,
                            }) => {
                                return QMark {
                                    token: token!(tCHAR, loc!(start, start + 1 + length), bytes),
                                };
                            }

                            Escape::SlashOctal(SlashOctal { byte, length })
                            | Escape::SlashX(SlashX { byte, length })
                            | Escape::SlashMetaCtrl(SlashMetaCtrl { byte, length })
                            | Escape::SlashByte(SlashByte { byte, length }) => {
                                return QMark {
                                    token: token!(tCHAR, loc!(start, start + 1 + length), byte),
                                };
                            }
                        },
                        Err(err) => match err {
                            EscapeError::SlashUError(SlashUError {
                                escaped_loc,
                                errors,
                                length,
                            }) => {
                                panic!(
                                    "SlashUError {:?} / {:?} / {:?}",
                                    escaped_loc, errors, length
                                );
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
                let c = std::str::from_utf8(buffer.slice(start + 1, end).expect("bug"))
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap();
                QMark {
                    token: token!(tCHAR, loc!(start, end), c),
                }
            }
            _ => QMark {
                token: token!(tEH, loc!(start, start + 1)),
            },
        }
    }
}

impl QMark {
    pub(crate) fn parse(buffer: &mut BufferWithCursor) -> Token {
        let start = buffer.pos();
        let QMark { token } = QMark::lookahead(buffer.for_lookahead_mut(), start);
        buffer.set_pos(token.loc.end);
        token
    }
}

#[cfg(test)]
mod tests {
    use crate::{testing::assert_lex, token::token};

    #[test]
    fn test_tEH() {
        assert_lex!(b"?", token!(tEH, loc!(0, 1)));
    }
    #[test]
    fn test_tCHAR_ascii() {
        assert_lex!(b"?a", token!(tCHAR, loc!(0, 2), b'a'));
    }
    #[test]
    fn test_tCHAR_multibyte() {
        assert_lex!("?字".as_bytes(), token!(tCHAR, loc!(0, 4), '字'));
    }
    #[test]
    fn test_tCHAR_slash_u() {
        assert_lex!(b"?\\u1234", token!(tCHAR, loc!(0, 7), '\u{1234}'));
    }
    #[test]
    fn test_tEH_and_ident() {
        assert_lex!(b"?ident", token!(tEH, loc!(0, 1)));
    }
}
