use super::Buffer;

pub(crate) trait Pattern {
    fn length(&self) -> usize;
    fn is_lookahead_of(&self, buffer: &Buffer) -> bool;
}

impl Pattern for u8 {
    fn length(&self) -> usize {
        1
    }

    fn is_lookahead_of(&self, buffer: &Buffer) -> bool {
        buffer.current_byte() == Some(*self)
    }
}

impl Pattern for &[u8] {
    fn length(&self) -> usize {
        self.len()
    }

    fn is_lookahead_of(&self, buffer: &Buffer) -> bool {
        if let Some(next) = buffer.input.get(buffer.pos..buffer.pos + self.length()) {
            self == &next
        } else {
            false
        }
    }
}

impl<const N: usize> Pattern for [u8; N] {
    fn length(&self) -> usize {
        N
    }

    fn is_lookahead_of(&self, buffer: &Buffer) -> bool {
        for i in 0..N {
            if buffer.byte_at(buffer.pos() + i) != Some(self[i]) {
                return false;
            }
        }
        true
    }
}
