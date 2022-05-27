use crate::lexer::buffer::{utf8::Utf8Char, Buffer};
use crate::token::{Loc, Token, TokenValue};

mod reserved_words;

use reserved_words::find_reserved_word;

pub(crate) fn is_identchar(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || !byte.is_ascii()
}

pub(crate) fn parse_ident<'a>(buffer: &mut Buffer<'a>) -> Token<'a> {
    let start = buffer.pos();

    let length = match lookahead_ident(buffer, start) {
        Some(length) => length,
        None => {
            todo!("handle ident that start with non-UTF-8 byte")
        }
    };

    buffer.set_pos(start + length);

    // lookahead to handle predicate/bang/setter method names
    match buffer.current_byte() {
        Some(b'!' | b'?') => {
            if buffer.byte_at(buffer.pos() + 1) == Some(b'=') {
                // `foo!=` means `foo !=`
                // `foo?=` means `foo ?=`
            } else {
                // append `!` or `?`
                buffer.skip_byte();
                return Token(
                    TokenValue::tFID(buffer.slice(start, buffer.pos())),
                    Loc(start, buffer.pos()),
                );
            }
        }
        Some(b'=') => {
            match buffer.byte_at(buffer.pos() + 1) {
                Some(b'~') => {
                    // `foo=~` means `foo =~`
                }
                Some(b'=') => {
                    // `foo==` means `foo==`
                }
                Some(b'>') => {
                    // `foo=>` means `foo => `
                }
                _ => {
                    // `foo=` setter, consume `'='
                    buffer.skip_byte();
                    return Token(
                        TokenValue::tIDENTIFIER(buffer.slice(start, buffer.pos())),
                        Loc(start, buffer.pos()),
                    );
                }
            }
        }
        _ => {}
    }

    // lookahead to handle `foo:` label
    // cases like
    //      1. foo?bar:baz
    //      2. def foo(bar:)
    // are handled on the parser level
    if buffer.current_byte() == Some(b':') {
        buffer.skip_byte();
        return Token(
            TokenValue::tLABEL(buffer.slice(start, buffer.pos())),
            Loc(start, buffer.pos()),
        );
    }

    let end = buffer.pos();
    let slice = buffer.slice(start, end);

    // there's a chance that it's a keyword
    if let Some(reserved_word) = find_reserved_word(slice) {
        return Token(reserved_word.token_value, Loc(start, end));
    }

    // otherwise it's just a plain identifier
    Token(TokenValue::tIDENTIFIER(slice), Loc(start, end))
}

// Returns None / Some(ident_length)
pub(crate) fn lookahead_ident<'a>(buffer: &Buffer<'a>, start: usize) -> Option<usize> {
    let mut end = start;

    loop {
        match buffer.byte_at(end) {
            Some(byte) if !is_identchar(byte) => {
                break;
            }
            None => {
                break;
            }
            _ => {}
        }

        match buffer.utf8_char_at(end) {
            Utf8Char::Valid(size) => {
                // keep consuming
                end += size
            }
            Utf8Char::Invalid => {
                break;
            }
            Utf8Char::EOF => {
                break;
            }
        }
    }

    if end == start {
        None
    } else {
        Some(end - start)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::assert_lex;

    #[test]
    fn test_is_identchar() {
        use super::is_identchar;

        assert!(is_identchar(b'0'));
        assert!(is_identchar(b'9'));
        assert!(is_identchar(b'a'));
        assert!(is_identchar(b'z'));
        assert!(is_identchar(b'A'));
        assert!(is_identchar(b'Z'));

        assert!(!is_identchar(b'('));
        assert!(!is_identchar(b'#'));
    }

    #[test]
    fn test_lookahead_ident() {
        use super::{lookahead_ident, Buffer};

        // ASCII ("foo")
        let buffer = Buffer::new(b" foo<2");
        assert_eq!(lookahead_ident(&buffer, 1), Some(3)); // captures "foo"

        // valid UTF-8 ("абв")
        let buffer = Buffer::new("абв".as_bytes());
        assert_eq!(lookahead_ident(&buffer, 0), Some(6)); // captures "абв"

        // ASCII ("foo") followed by malformed bytes (208, 0)
        let buffer = Buffer::new(&[b'f', b'o', b'o', 208, 0]);
        assert_eq!(lookahead_ident(&buffer, 0), Some(3)); // captures "foo"

        // UTF-8 ("абв") followed by malformed bytes (208, 0)
        let buffer = Buffer::new(&[208, 176, 208, 177, 208, 177, 208, 0]);
        assert_eq!(lookahead_ident(&buffer, 0), Some(6)); // captures "абв"
    }

    assert_lex!(test_tIDENTIFIER_plain, b"foo", tIDENTIFIER(b"foo"), 0..3);

    assert_lex!(
        test_tIDENTIFIER_multibyte,
        b"\xD0\xB0\xD0\xB1\xD0\xB2+",
        tIDENTIFIER(b"\xD0\xB0\xD0\xB1\xD0\xB2"),
        0..6
    );

    assert_lex!(test_tFID_predicate, b"foo?", tFID(b"foo?"), 0..4);
    assert_lex!(test_tFID_predicate_eq, b"foo?=", tIDENTIFIER(b"foo"), 0..3);
    assert_lex!(test_tFID_bang, b"foo!", tFID(b"foo!"), 0..4);
    assert_lex!(test_tFID_bang_eq, b"foo!=", tIDENTIFIER(b"foo"), 0..3);
    assert_lex!(test_tIDENTIFIER_setter, b"foo=", tIDENTIFIER(b"foo="), 0..4);
    assert_lex!(
        test_tIDENTIFIER_setter_tilde,
        b"foo=~",
        tIDENTIFIER(b"foo"),
        0..3
    );
    assert_lex!(
        test_tIDENTIFIER_setter_eq,
        b"foo==",
        tIDENTIFIER(b"foo"),
        0..3
    );
    assert_lex!(
        test_tIDENTIFIER_setter_gt,
        b"foo=>",
        tIDENTIFIER(b"foo"),
        0..3
    );

    assert_lex!(test_tLABEL, b"foo:", tLABEL(b"foo:"), 0..4);

    assert_lex!(test_reserved_word, b"if", kIF, 0..2);
}
