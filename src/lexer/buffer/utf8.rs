use crate::lexer::buffer::Buffer;

// UTF-8 support

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Utf8Char {
    Valid(usize),
    Invalid,
    EOF,
}

impl<'a> Buffer<'a> {
    // Returns size of the UTF-8 char
    pub(crate) fn utf8_char_at(&self, mut idx: usize) -> Utf8Char {
        if let Some(c) = self.byte_at(idx) {
            let size = if c & 0x80 == 0 {
                1
            } else if c & 0xE0 == 0xC0 {
                2
            } else if c & 0xF0 == 0xE0 {
                3
            } else if c & 0xF8 == 0xF0 {
                4
            } else {
                // malformed
                return Utf8Char::Invalid;
            };

            let slice = self.slice(idx, idx + size);
            match std::str::from_utf8(slice) {
                Ok(_) => Utf8Char::Valid(size),
                Err(_) => Utf8Char::Invalid,
            }
        } else {
            Utf8Char::EOF
        }
    }

    pub(crate) fn current_utf8_char(&self) -> Utf8Char {
        self.utf8_char_at(self.pos)
    }
}

#[test]
fn test_utf8_char_at_ascii() {
    let buffer = Buffer::new(b"foo");
    assert_eq!(buffer.utf8_char_at(0), Utf8Char::Valid(1));
    assert_eq!(buffer.utf8_char_at(1), Utf8Char::Valid(1));
    assert_eq!(buffer.utf8_char_at(2), Utf8Char::Valid(1));
    assert_eq!(buffer.utf8_char_at(3), Utf8Char::EOF);
}

#[test]
fn test_utf8_char_at_multibyte() {
    let buffer = Buffer::new("абв".as_bytes());

    // at the beginning of "а"
    assert_eq!(buffer.utf8_char_at(0), Utf8Char::Valid(2));
    // in the middle of "а"
    assert_eq!(buffer.utf8_char_at(1), Utf8Char::Invalid);

    // at the beginning of "б"
    assert_eq!(buffer.utf8_char_at(2), Utf8Char::Valid(2));
    // in the middle of "б"
    assert_eq!(buffer.utf8_char_at(3), Utf8Char::Invalid);

    // at the beginning of "в"
    assert_eq!(buffer.utf8_char_at(4), Utf8Char::Valid(2));
    // in the middle of "в"
    assert_eq!(buffer.utf8_char_at(5), Utf8Char::Invalid);

    // at EOF
    assert_eq!(buffer.utf8_char_at(6), Utf8Char::EOF);
}
