use crate::lexer::buffer::{Buffer, Lookahead};

pub(crate) struct CodepointWide {
    pub(crate) length: usize,
}

pub(crate) enum CodepointWideError {
    NonHexErr { length: usize },
    TooLong { length: usize },
}

impl Lookahead for CodepointWide {
    type Output = Result<Self, CodepointWideError>;

    fn lookahead(buffer: &Buffer, start: usize) -> Self::Output {
        let mut end = start;
        let mut valid = true;
        loop {
            match buffer.byte_at(end) {
                Some(b' ' | b'\t') => break,
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
            return Err(CodepointWideError::NonHexErr {
                length: end - start,
            });
        }

        let length = end - start;
        if length > 6 {
            return Err(CodepointWideError::TooLong { length });
        }

        Ok(CodepointWide { length })
    }
}
