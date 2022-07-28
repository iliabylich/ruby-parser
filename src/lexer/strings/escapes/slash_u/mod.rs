mod short;
use short::{CodepointShort, CodepointShortError};

mod wide;
use wide::{CodepointWide, CodepointWideError};

use crate::{
    lexer::buffer::{scan_while_matches_pattern, Buffer, LookaheadResult},
    Loc,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SlashU {
    Short { codepoint: char, length: usize },
    Wide { escaped_loc: Loc, length: usize },
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SlashUError {
    pub(crate) escaped_loc: Loc,
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

impl SlashU {
    pub(crate) fn lookahead(
        buffer: &mut Buffer,
        start: usize,
    ) -> Result<Option<Self>, SlashUError> {
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
            let mut escaped_loc = Loc { start: 0, end: 0 };

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
                        read_codepoint_wide(buffer, pos, pos + length, &mut escaped_loc);
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
                    escaped_loc,
                    length: pos - start,
                }));
            } else {
                return Err(SlashUError {
                    escaped_loc,
                    errors,
                    length: pos - start,
                });
            }
        } else {
            // short
            match CodepointShort::lookahead(buffer, pos) {
                Ok(CodepointShort { length }) => {
                    debug_assert_eq!(length, 4);

                    let codepoint =
                        read_codepoint_short(buffer.slice(pos, pos + length).expect("bug"));
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
                        escaped_loc: Loc { start: 0, end: 0 },
                        errors,
                        length: pos - start,
                    });
                }
            }
        }
    }
}

fn read_codepoint_short(hex_bytes: &[u8]) -> char {
    let s = std::str::from_utf8(hex_bytes).unwrap();
    let codepoint = u32::from_str_radix(s, 16).unwrap();
    char::from_u32(codepoint).unwrap()
}

fn read_codepoint_wide(buffer: &mut Buffer, start: usize, end: usize, escaped_loc: &mut Loc) {
    let hex_bytes = buffer.slice(start, end).expect("bug");
    let s = std::str::from_utf8(hex_bytes).unwrap();
    let codepoint = u32::from_str_radix(s, 16).unwrap();
    let c = char::from_u32(codepoint).unwrap();
    let s = String::from(c);
    let mut bytes = s.into_bytes();

    let start = buffer.unescaped_len();
    buffer.append_unesscaped(&mut bytes);
    let end = buffer.unescaped_len();

    if escaped_loc.is_empty() {
        *escaped_loc = Loc { start, end }
    } else {
        escaped_loc.end = end;
    }
}

#[cfg(test)]
mod tests;
