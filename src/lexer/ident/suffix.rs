use crate::lexer::buffer::Buffer;

pub(crate) struct IdentSuffix {
    pub(crate) byte: u8,
}

impl IdentSuffix {
    pub(crate) fn lookahead(buffer: &Buffer, start: usize) -> Option<Self> {
        match buffer.byte_at(start) {
            Some(suffix @ (b'!' | b'?')) => {
                if buffer.byte_at(start + 1) == Some(b'=') {
                    // `foo!=` means `foo !=`
                    // `foo?=` means `foo ?=`
                    None
                } else {
                    // append `!` or `?`
                    Some(IdentSuffix { byte: suffix })
                }
            }
            Some(suffix @ b'=') => {
                match buffer.byte_at(start + 1) {
                    Some(b'~') => {
                        // `foo=~` means `foo =~`
                        None
                    }
                    Some(b'=') => {
                        // `foo==` means `foo==`
                        None
                    }
                    Some(b'>') => {
                        // `foo=>` means `foo => `
                        None
                    }
                    _ => {
                        // `foo=` setter, consume `'='
                        Some(IdentSuffix { byte: suffix })
                    }
                }
            }
            _ => None,
        }
    }
}
