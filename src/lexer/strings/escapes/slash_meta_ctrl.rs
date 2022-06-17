use crate::lexer::buffer::{Buffer, Lookahead};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SlashMetaCtrl {
    codepoint: u8,
    length: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SlashMetaCtrlErr {
    length: usize,
}

impl<'a> Lookahead<'a> for SlashMetaCtrl {
    type Output = Result<Option<Self>, SlashMetaCtrlErr>;

    // \C-\M-f
    // \C-\f
    // \c\M-f
    // \cf
    // \M-\cf
    // \M-f
    fn lookahead(buffer: &Buffer<'a>, start: usize) -> Self::Output {
        if buffer.byte_at(start) != Some(b'\\') {
            return Ok(None);
        }

        let hex_or_maybe_escaped_char = |at: usize| {
            let prefix_length = at - start;

            let result = try_escaped_char(buffer, at)
                .or_else(|| try_hex2(buffer, at))
                .or_else(|| try_byte(buffer, at))
                .ok_or_else(|| emit_err(prefix_length))
                .map(|mut scan| {
                    scan.length += prefix_length;
                    scan
                });

            result
        };

        let mut length = 1; // `\`

        let lookahead = if buffer.lookahead(start + length, b"C-") {
            length += 2;

            if buffer.lookahead(start + length, b"\\M-") {
                // 1. \C-\M-d
                length += 3;

                hex_or_maybe_escaped_char(start + length)?
                    .map(slash_c)
                    .map(slash_m)
            } else {
                // 2. \C-d
                hex_or_maybe_escaped_char(start + length)?
                    .map(unescape_byte)
                    .map(slash_c)
            }
        } else if buffer.lookahead(start + length, b"c") {
            length += 1;

            if buffer.lookahead(start + length, b"\\M-") {
                // 1. \c\M-d
                length += 3;

                hex_or_maybe_escaped_char(start + length)?
                    .map(slash_c)
                    .map(slash_m)
            } else {
                // 2. \cd
                hex_or_maybe_escaped_char(start + length)?.map(slash_c)
            }
        } else if buffer.lookahead(start + length, b"M-") {
            length += 2;

            if buffer.lookahead(start + length, b"\\c") {
                // 1. \M-\cd
                length += 2;

                hex_or_maybe_escaped_char(start + length)?
                    .map(slash_c)
                    .map(slash_m)
            } else {
                // 2. \M-d
                hex_or_maybe_escaped_char(start + length)?.map(slash_m)
            }
        } else {
            // just a `\` without `c`/`C`/`M`, probably a different escape sequence
            return Ok(None);
        };

        Ok(Some(lookahead))
    }
}

fn emit_err(length: usize) -> SlashMetaCtrlErr {
    SlashMetaCtrlErr { length }
}

fn unescape_byte(byte: u8) -> u8 {
    match byte {
        b'a' => 7,      // ?\a
        b'b' => 8,      // ?\b
        b'e' => 27,     // ?\e
        b'f' => 12,     // ?\f
        b'n' => 10,     // ?\n
        b'r' => 13,     // ?\r
        b's' => 32,     // ?\s
        b't' => 9,      // ?\t
        b'v' => 11,     // ?\v
        b'\\' => b'\\', // ?\\,
        _ => byte,
    }
}

fn slash_c(byte: u8) -> u8 {
    byte & 0x9f
}
fn slash_m(byte: u8) -> u8 {
    byte | 0x80
}

impl SlashMetaCtrl {
    fn map<F: FnOnce(u8) -> u8>(mut self, f: F) -> Self {
        self.codepoint = f(self.codepoint);
        self
    }
}

fn try_escaped_char(buffer: &Buffer, at: usize) -> Option<SlashMetaCtrl> {
    if buffer.byte_at(at)? == b'\\' {
        let byte = buffer.byte_at(at + 1)?;
        return Some(SlashMetaCtrl {
            codepoint: unescape_byte(byte),
            length: 2,
        });
    }
    None
}

fn try_hex(buffer: &Buffer, at: usize) -> Option<u8> {
    let hex = match buffer.byte_at(at) {
        Some(byte @ b'0'..=b'9') => Some(byte - b'0'),
        Some(byte @ b'a'..=b'f') => Some(byte - b'a'),
        Some(byte @ b'A'..=b'F') => Some(byte - b'A'),
        _ => None,
    };
    return hex;
}

fn try_hex2(buffer: &Buffer, at: usize) -> Option<SlashMetaCtrl> {
    let byte1 = try_hex(buffer, at)? << 4;
    let byte2 = try_hex(buffer, at + 1)?;
    Some(SlashMetaCtrl {
        codepoint: (byte1 << 4) & byte2,
        length: 2,
    })
}

fn try_byte(buffer: &Buffer, at: usize) -> Option<SlashMetaCtrl> {
    let byte = buffer.byte_at(at)?;
    Some(SlashMetaCtrl {
        codepoint: byte,
        length: 1,
    })
}

macro_rules! assert_lookahead {
    (test = $test:ident, input = $input:expr, output = $output:expr) => {
        #[test]
        fn $test() {
            #[allow(unused_imports)]
            use crate::lexer::{
                buffer::{Buffer, Lookahead},
                strings::escapes::{SlashMetaCtrl, SlashMetaCtrlErr},
            };
            let buffer = Buffer::new($input);
            let lookahead = SlashMetaCtrl::lookahead(&buffer, 0);
            assert_eq!(lookahead, $output);
        }
    };
}

assert_lookahead!(
    test = test_lookahead_nothing,
    input = b"foobar",
    output = Ok(None)
);

mod slash_big_c {
    // 1. \C-\M-d
    assert_lookahead!(
        test = test_dash_slash_big_m_dash_codepoint,
        input = b"\\C-\\M-d",
        output = Ok(Some(SlashMetaCtrl {
            codepoint: 132,
            length: 7
        }))
    );
    // 2. \C-\d
    assert_lookahead!(
        test = test_dash_slash_codepoint,
        input = b"\\C-\\d",
        output = Ok(Some(SlashMetaCtrl {
            codepoint: 4,
            length: 5
        }))
    );

    // \C-\M-f
    assert_lookahead!(
        test = test_dash_slash_big_m_dash_escaped_codepoint,
        input = b"\\C-\\M-f",
        output = Ok(Some(SlashMetaCtrl {
            codepoint: 134,
            length: 7
        }))
    );
    // \C-\d
    assert_lookahead!(
        test = test_dash_slash_escaped_codepoint,
        input = b"\\C-\\f",
        output = Ok(Some(SlashMetaCtrl {
            codepoint: 12,
            length: 5
        }))
    );

    assert_lookahead!(
        test = test_invalid_eof,
        input = b"\\C-",
        output = Err(SlashMetaCtrlErr { length: 3 })
    );
}

mod slash_low_c {
    // 1. \c\M-d
    assert_lookahead!(
        test = test_slash_big_m_dash_codepoint,
        input = b"\\c\\M-d",
        output = Ok(Some(SlashMetaCtrl {
            codepoint: 132,
            length: 6
        }))
    );
    // 2. \cd
    assert_lookahead!(
        test = test_codepoint,
        input = b"\\cd",
        output = Ok(Some(SlashMetaCtrl {
            codepoint: 4,
            length: 3
        }))
    );

    // \c\M-f
    assert_lookahead!(
        test = test_slash_big_m_dash_escaped_codepoint,
        input = b"\\c\\M-f",
        output = Ok(Some(SlashMetaCtrl {
            codepoint: 134,
            length: 6
        }))
    );
    // 2. \cf
    assert_lookahead!(
        test = test_escaped_codepoint,
        input = b"\\cf",
        output = Ok(Some(SlashMetaCtrl {
            codepoint: 6,
            length: 3
        }))
    );

    assert_lookahead!(
        test = test_invalid_eof,
        input = b"\\c",
        output = Err(SlashMetaCtrlErr { length: 2 })
    );
}

mod slash_big_m {
    // 1. \M-\cd
    assert_lookahead!(
        test = test_dash_slash_low_c_codepoint,
        input = b"\\M-\\cd",
        output = Ok(Some(SlashMetaCtrl {
            codepoint: 132,
            length: 6
        }))
    );
    // 2. \M-d
    assert_lookahead!(
        test = test_dash_codepoint,
        input = b"\\M-d",
        output = Ok(Some(SlashMetaCtrl {
            codepoint: 228,
            length: 4
        }))
    );

    // \M-\cf
    assert_lookahead!(
        test = test_dash_slash_low_c_escaped_codepoint,
        input = b"\\M-\\cf",
        output = Ok(Some(SlashMetaCtrl {
            codepoint: 134,
            length: 6
        }))
    );
    // \M-f
    assert_lookahead!(
        test = test_dash_escaped_codepoint,
        input = b"\\M-f",
        output = Ok(Some(SlashMetaCtrl {
            codepoint: 230,
            length: 4
        }))
    );

    assert_lookahead!(
        test = test_invalid_eof,
        input = b"\\M-",
        output = Err(SlashMetaCtrlErr { length: 3 })
    );
}
