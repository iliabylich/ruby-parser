use crate::buffer::Pattern;

#[derive(Debug)]
pub struct Buffer {
    bytes: Vec<u8>,
    pub(crate) unescaped_bytes: Vec<u8>,
}

impl Buffer {
    pub(crate) fn new(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
            unescaped_bytes: vec![],
        }
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub(crate) fn slice(&self, start: usize, end: usize) -> Option<&[u8]> {
        self.bytes.get(start..end)
    }

    pub(crate) fn byte_at(&self, idx: usize) -> Option<u8> {
        self.bytes.get(idx).map(|byte| *byte)
    }

    pub(crate) fn lookahead<P: Pattern>(&self, start: usize, pattern: &P) -> bool {
        match self.bytes.get(start..) {
            Some(bytes) => pattern.is_lookahead_of(bytes),
            None => false,
        }
    }

    pub(crate) fn unescaped_len(&self) -> usize {
        self.unescaped_bytes.len()
    }
    pub(crate) fn append_unesscaped(&mut self, unescaped: &mut Vec<u8>) {
        self.unescaped_bytes.append(unescaped)
    }

    pub(crate) fn unescaped_slice_at(&self, start: usize, end: usize) -> Option<&[u8]> {
        self.unescaped_bytes.get(start..end)
    }
}
