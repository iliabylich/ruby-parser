pub(crate) trait Pattern {
    fn length(&self) -> usize;
    fn is_lookahead_of(&self, bytes: &[u8]) -> bool;
}

impl Pattern for u8 {
    fn length(&self) -> usize {
        1
    }

    fn is_lookahead_of(&self, bytes: &[u8]) -> bool {
        bytes.get(0) == Some(self)
    }
}

impl Pattern for &[u8] {
    fn length(&self) -> usize {
        self.len()
    }

    fn is_lookahead_of(&self, bytes: &[u8]) -> bool {
        if let Some(next) = bytes.get(0..self.length()) {
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

    fn is_lookahead_of(&self, bytes: &[u8]) -> bool {
        for i in 0..N {
            if bytes.get(i) != Some(&self[i]) {
                return false;
            }
        }
        true
    }
}
