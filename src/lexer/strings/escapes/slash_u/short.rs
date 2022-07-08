use crate::lexer::buffer::{Buffer, Lookahead};

pub(crate) struct CodepointShort {
    pub(crate) length: usize,
}

pub(crate) struct CodepointShortError {
    pub(crate) length: usize,
}

impl Lookahead for CodepointShort {
    type Output = Result<Self, CodepointShortError>;

    fn lookahead(buffer: &Buffer, start: usize) -> Self::Output {
        let mut length = 0;
        for i in 1..=4 {
            match buffer.byte_at(start + i - 1) {
                Some(b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F') => length = i,
                _ => break,
            }
        }

        if length != 4 {
            return Err(CodepointShortError { length });
        }

        Ok(CodepointShort { length })
    }
}
