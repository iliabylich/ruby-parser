mod short;
use short::{CodepointShort, CodepointShortError};

mod wide;
use wide::{CodepointWide, CodepointWideError};

use crate::lexer::buffer::{scan_while_matches_pattern, Buffer, Lookahead, LookaheadResult};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SlashU {
    Short { bytes: Vec<u8>, length: usize },
    Wide { bytes: Vec<u8>, length: usize },
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SlashUError {
    pub(crate) valid_bytes: Option<Vec<u8>>,
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
            let mut bytes = vec![];
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
                        read_codepoint(buffer.slice(pos, pos + length).expect("bug"), &mut bytes);
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
                    bytes,
                    length: pos - start,
                }));
            } else {
                let codepoints = if bytes.is_empty() { None } else { Some(bytes) };
                return Err(SlashUError {
                    valid_bytes: codepoints,
                    errors,
                    length: pos - start,
                });
            }
        } else {
            // short
            let mut bytes = vec![];

            match CodepointShort::lookahead(buffer, pos) {
                Ok(CodepointShort { length }) => {
                    debug_assert_eq!(length, 4);

                    read_codepoint(buffer.slice(pos, pos + length).expect("bug"), &mut bytes);
                    pos += length;
                }
                Err(CodepointShortError { length }) => {
                    errors.push(SlashUPerCodepointError::Expected4Got { start: pos, length });
                    pos += length;
                }
            }

            if !bytes.is_empty() {
                return Ok(Some(SlashU::Short {
                    bytes,
                    length: pos - start,
                }));
            } else {
                return Err(SlashUError {
                    valid_bytes: None,
                    errors,
                    length: pos - start,
                });
            }
        }
    }
}

fn read_codepoint(hex_bytes: &[u8], dest: &mut Vec<u8>) {
    let s = std::str::from_utf8(hex_bytes).unwrap();
    let codepoint = u32::from_str_radix(s, 16).unwrap();
    let c = char::from_u32(codepoint).unwrap();

    let mut buf = vec![0; c.len_utf8()];
    c.encode_utf8(&mut buf);

    dest.append(&mut buf);
}

#[cfg(test)]
mod tests;
