use crate::lexer::buffer::Buffer;
use crate::token::Token;

pub(crate) fn is_identchar(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || !byte.is_ascii()
}

pub(crate) fn parse_ident<'a>(_buffer: &mut Buffer<'a>) -> Token<'a> {
    todo!("parse_ident")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_identchar() {
        assert!(is_identchar(b'0'));
        assert!(is_identchar(b'9'));
        assert!(is_identchar(b'a'));
        assert!(is_identchar(b'z'));
        assert!(is_identchar(b'A'));
        assert!(is_identchar(b'Z'));

        assert!(!is_identchar(b'('));
        assert!(!is_identchar(b'#'));
    }
}
