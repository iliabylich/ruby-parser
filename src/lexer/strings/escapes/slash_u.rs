use crate::lexer::buffer::{scan_while_matches_pattern, Buffer, Lookahead, LookaheadResult};

pub(crate) struct SlashU;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LooakeadhSlashUResult {
    Short {
        codepoint: char,
        length: usize,
    },
    Wide {
        codepoints: Box<[char]>,
        length: usize,
    },
    Err {
        codepoints: Option<Box<[char]>>,
        errors: Box<[SlashUError]>,
        length: usize,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum SlashUError {
    Expected4Got { start: usize, length: usize },
    TooLong { start: usize, length: usize },
    NonHex { start: usize, length: usize },
    NoRCurly { start: usize },
}

impl<'a> Lookahead<'a> for SlashU {
    type Output = LooakeadhSlashUResult;

    fn lookahead(buffer: &Buffer<'a>, start: usize) -> Self::Output {
        if !buffer.lookahead(b"\\u") {
            return LooakeadhSlashUResult::Err {
                codepoints: None,
                errors: Box::new([]),
                length: 0,
            };
        }

        let mut wide = false;
        let mut pos = start + 2;

        if buffer.byte_at(pos) == Some(b'{') {
            wide = true;
            pos += 1;
        }

        let mut errors = vec![];

        if wide {
            let mut codepoints = vec![];
            loop {
                if let LookaheadResult::Some { length } =
                    scan_while_matches_pattern!(buffer, pos, b' ')
                {
                    pos += length;
                }

                match CodepointWide::lookahead(buffer, pos) {
                    LookaheadCodepointWideResult::Ok { length: 0 } => {
                        // EOF
                        break;
                    }
                    LookaheadCodepointWideResult::Ok { length } => {
                        read_codepoint(buffer.slice(pos, pos + length), &mut codepoints);
                        pos += length;
                    }
                    LookaheadCodepointWideResult::NonHexErr { length } => {
                        errors.push(SlashUError::NonHex { start: pos, length });
                        pos += length;
                    }
                    LookaheadCodepointWideResult::TooLong { length } => {
                        errors.push(SlashUError::TooLong { start: pos, length });
                        pos += length;
                    }
                }
            }

            // track trailing '}' if possible
            if buffer.byte_at(pos) == Some(b'}') {
                pos += 1;
            } else {
                errors.push(SlashUError::NoRCurly { start: pos });
            }

            if errors.is_empty() {
                return LooakeadhSlashUResult::Wide {
                    codepoints: codepoints.into_boxed_slice(),
                    length: pos - start,
                };
            } else {
                let codepoints = if codepoints.is_empty() {
                    None
                } else {
                    Some(codepoints.into_boxed_slice())
                };
                return LooakeadhSlashUResult::Err {
                    codepoints,
                    errors: errors.into_boxed_slice(),
                    length: pos - start,
                };
            }
        } else {
            // short
            let mut codepoints = vec![];

            match CodepointShort::lookahead(buffer, pos) {
                LookaheadCodepointShort::Ok { length } => {
                    debug_assert_eq!(length, 4);

                    read_codepoint(buffer.slice(pos, pos + length), &mut codepoints);
                    pos += length;
                }
                LookaheadCodepointShort::Expected4GotErr { length } => {
                    errors.push(SlashUError::Expected4Got { start: pos, length });
                    pos += length;
                }
            }

            if let Some(codepoint) = codepoints.into_iter().next() {
                return LooakeadhSlashUResult::Short {
                    codepoint,
                    length: pos - start,
                };
            } else {
                return LooakeadhSlashUResult::Err {
                    codepoints: None,
                    errors: errors.into_boxed_slice(),
                    length: pos - start,
                };
            }
        }
    }
}

struct CodepointWide;

#[derive(Debug)]
enum LookaheadCodepointWideResult {
    Ok { length: usize },
    NonHexErr { length: usize },
    TooLong { length: usize },
}

impl<'a> Lookahead<'a> for CodepointWide {
    type Output = LookaheadCodepointWideResult;

    fn lookahead(buffer: &Buffer<'a>, start: usize) -> Self::Output {
        dbg!(start);
        let mut end = start;
        let mut valid = true;
        loop {
            match buffer.byte_at(end) {
                Some(b' ') => break,
                Some(b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F') => {
                    end += 1;
                }
                None => break,
                Some(b'}') => break,
                _ => {
                    end += 1;
                    valid = false;
                }
            }
        }

        if !valid {
            return LookaheadCodepointWideResult::NonHexErr {
                length: end - start,
            };
        }

        let length = end - start;
        if length > 6 {
            return LookaheadCodepointWideResult::TooLong { length };
        }

        LookaheadCodepointWideResult::Ok { length }
    }
}

struct CodepointShort;

#[derive(Debug)]
enum LookaheadCodepointShort {
    Ok { length: usize },
    Expected4GotErr { length: usize },
}

impl<'a> Lookahead<'a> for CodepointShort {
    type Output = LookaheadCodepointShort;

    fn lookahead(buffer: &Buffer<'a>, start: usize) -> Self::Output {
        let mut length = 0;
        for i in 1..=4 {
            match buffer.byte_at(start + i - 1) {
                Some(b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F') => length = i,
                _ => break,
            }
        }

        if length != 4 {
            return LookaheadCodepointShort::Expected4GotErr { length };
        }

        LookaheadCodepointShort::Ok { length }
    }
}

fn read_codepoint(hex_bytes: &[u8], dest: &mut Vec<char>) {
    dbg!(hex_bytes);
    let s = std::str::from_utf8(hex_bytes).unwrap();
    let codepoint = u32::from_str_radix(s, 16).unwrap();
    let c = char::from_u32(codepoint).unwrap();
    dest.push(c)
    // dbg!(c.len_utf8());
    // let start = dest.len();
    // for _ in 0..c.len_utf8() {
    //     dest.push(0);
    // }
    // dbg!(start);
    // c.encode_utf8(&mut dest[start..]);
}

macro_rules! assert_scans {
    (test = $test:ident, input = $input:expr, output = $output:expr) => {
        #[test]
        fn $test() {
            let buffer = Buffer::new($input);
            assert_eq!(SlashU::lookahead(&buffer, 0), $output);
        }
    };
}

assert_scans!(
    test = test_slash_u_nothing,
    input = b"foobar",
    output = LooakeadhSlashUResult::Err {
        codepoints: None,
        errors: Box::new([]),
        length: 0
    }
);

// short
assert_scans!(
    test = test_slash_u_short_valid,
    input = b"\\u123456",
    output = LooakeadhSlashUResult::Short {
        codepoint: '\u{1234}',
        length: 6
    }
);
assert_scans!(
    test = test_slash_u_short_invalid,
    input = b"\\uxxxxxx",
    output = LooakeadhSlashUResult::Err {
        codepoints: None,
        errors: Box::new([SlashUError::Expected4Got {
            start: 2,
            length: 0
        }]),
        length: 2
    }
);

// wide
assert_scans!(
    test = test_slash_u_wide_single_codepoint_valid,
    input = b"\\u{1234}",
    output = LooakeadhSlashUResult::Wide {
        codepoints: Box::new(['\u{1234}']),
        length: 8
    }
);
assert_scans!(
    test = test_slash_u_wide_multiple_codepoint_valid,
    input = b"\\u{ 1234   4321  }",
    output = LooakeadhSlashUResult::Wide {
        codepoints: Box::new(['\u{1234}', '\u{4321}']),
        length: 18
    }
);
assert_scans!(
    test = test_slash_u_curly_unterminated,
    input = b"\\u{foo123",
    output = LooakeadhSlashUResult::Err {
        codepoints: None,
        errors: Box::new([
            SlashUError::NonHex {
                start: 3,
                length: 6
            },
            SlashUError::NoRCurly { start: 9 }
        ]),
        length: 9
    }
);
