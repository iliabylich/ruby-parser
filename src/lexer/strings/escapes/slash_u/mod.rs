mod short;
use short::{CodepointShort, CodepointShortError};

mod wide;
use wide::{CodepointWide, CodepointWideError};

use crate::lexer::buffer::{scan_while_matches_pattern, Buffer, Lookahead, LookaheadResult};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SlashU {
    Short {
        codepoint: char,
        length: usize,
    },
    Wide {
        codepoints: Vec<char>,
        length: usize,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SlashUError {
    pub(crate) codepoints: Option<Vec<char>>,
    pub(crate) errors: Vec<SlashUPerCodepointError>,
    pub(crate) length: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SlashUPerCodepointError {
    Expected4Got { start: usize, length: usize },
    TooLong { start: usize, length: usize },
    NonHex { start: usize, length: usize },
    NoRCurly { start: usize },
}

impl Lookahead for SlashU {
    type Output = Result<Option<Self>, SlashUError>;

    fn lookahead(buffer: &Buffer, start: usize) -> Self::Output {
        if buffer.byte_at(start) != Some(b'\\') || buffer.byte_at(start + 1) != Some(b'u') {
            return Ok(None);
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
                    scan_while_matches_pattern!(buffer, pos, b' ' | b'\t')
                {
                    pos += length;
                }

                match CodepointWide::lookahead(buffer, pos) {
                    Ok(CodepointWide { length: 0 }) => {
                        // EOF
                        break;
                    }
                    Ok(CodepointWide { length }) => {
                        read_codepoint_to(
                            buffer.slice(pos, pos + length).expect("bug"),
                            &mut codepoints,
                        );
                        pos += length;
                    }
                    Err(CodepointWideError::NonHexErr { length }) => {
                        errors.push(SlashUPerCodepointError::NonHex { start: pos, length });
                        pos += length;
                    }
                    Err(CodepointWideError::TooLong { length }) => {
                        errors.push(SlashUPerCodepointError::TooLong { start: pos, length });
                        pos += length;
                    }
                }
            }

            // track trailing '}' if possible
            if buffer.byte_at(pos) == Some(b'}') {
                pos += 1;
            } else {
                errors.push(SlashUPerCodepointError::NoRCurly { start: pos });
            }

            if errors.is_empty() {
                return Ok(Some(SlashU::Wide {
                    codepoints,
                    length: pos - start,
                }));
            } else {
                let codepoints = if codepoints.is_empty() {
                    None
                } else {
                    Some(codepoints)
                };
                return Err(SlashUError {
                    codepoints,
                    errors,
                    length: pos - start,
                });
            }
        } else {
            // short
            match CodepointShort::lookahead(buffer, pos) {
                Ok(CodepointShort { length }) => {
                    debug_assert_eq!(length, 4);

                    let codepoint = read_codepoint(buffer.slice(pos, pos + length).expect("bug"));
                    pos += length;

                    return Ok(Some(SlashU::Short {
                        codepoint,
                        length: pos - start,
                    }));
                }
                Err(CodepointShortError { length }) => {
                    errors.push(SlashUPerCodepointError::Expected4Got { start: pos, length });
                    pos += length;

                    return Err(SlashUError {
                        codepoints: None,
                        errors,
                        length: pos - start,
                    });
                }
            }
        }
    }
}

fn read_codepoint(hex_bytes: &[u8]) -> char {
    let s = std::str::from_utf8(hex_bytes).unwrap();
    let codepoint = u32::from_str_radix(s, 16).unwrap();
    char::from_u32(codepoint).unwrap()
}

fn read_codepoint_to(hex_bytes: &[u8], dest: &mut Vec<char>) {
    let c = read_codepoint(hex_bytes);
    dest.push(c);
}

#[cfg(test)]
mod tests;
