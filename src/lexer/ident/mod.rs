use crate::lexer::{
    assert_lex,
    buffer::{utf8::Utf8Char, Buffer, Lookahead, LookaheadResult},
};
use crate::token::{token, Loc, Token};

mod reserved_words;

use reserved_words::find_reserved_word;

pub(crate) struct Ident;

impl Lookahead for Ident {
    type Output = LookaheadResult;

    fn lookahead(buffer: &Buffer, start: usize) -> Self::Output {
        let mut end = start;

        loop {
            match buffer.byte_at(end) {
                Some(byte) if !Self::is_identchar(byte) => {
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
            LookaheadResult::None
        } else {
            LookaheadResult::Some {
                length: end - start,
            }
        }
    }
}

impl Ident {
    pub(crate) fn is_identchar(byte: u8) -> bool {
        byte.is_ascii_alphanumeric() || byte == b'_' || !byte.is_ascii()
    }

    pub(crate) fn parse(buffer: &mut Buffer) -> Token {
        let start = buffer.pos();

        let length = match Ident::lookahead(buffer, start) {
            LookaheadResult::Some { length } => length,
            LookaheadResult::None => {
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
                    return token!(tFID, start, buffer.pos());
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
                        return token!(tIDENTIFIER, start, buffer.pos());
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
            return token!(tLABEL, start, buffer.pos());
        }

        let end = buffer.pos();
        let slice = buffer.slice(start, end);

        // there's a chance that it's a keyword
        if let Some(reserved_word) = find_reserved_word(slice) {
            return Token(reserved_word.token_value, Loc(start, end));
        }

        // otherwise it's just a plain identifier
        token!(tIDENTIFIER, start, end)
    }
}

#[test]
fn test_is_identchar() {
    assert!(Ident::is_identchar(b'0'));
    assert!(Ident::is_identchar(b'9'));
    assert!(Ident::is_identchar(b'a'));
    assert!(Ident::is_identchar(b'z'));
    assert!(Ident::is_identchar(b'A'));
    assert!(Ident::is_identchar(b'Z'));

    assert!(!Ident::is_identchar(b'('));
    assert!(!Ident::is_identchar(b'#'));
}

#[test]
fn test_lookahead_ident() {
    // ASCII ("foo")
    let buffer = Buffer::new(b" foo<2");
    assert_eq!(
        Ident::lookahead(&buffer, 1),
        LookaheadResult::Some { length: 3 }
    ); // captures "foo"

    // valid UTF-8 ("абв")
    let buffer = Buffer::new("абв".as_bytes());
    assert_eq!(
        Ident::lookahead(&buffer, 0),
        LookaheadResult::Some { length: 6 }
    ); // captures "абв"

    // ASCII ("foo") followed by malformed bytes (208, 0)
    let buffer = Buffer::new(&[b'f', b'o', b'o', 208, 0]);
    assert_eq!(
        Ident::lookahead(&buffer, 0),
        LookaheadResult::Some { length: 3 }
    ); // captures "foo"

    // UTF-8 ("абв") followed by malformed bytes (208, 0)
    let buffer = Buffer::new(&[208, 176, 208, 177, 208, 177, 208, 0]);
    assert_eq!(
        Ident::lookahead(&buffer, 0),
        LookaheadResult::Some { length: 6 }
    ); // captures "абв"
}

assert_lex!(test_tIDENTIFIER_plain, b"foo", tIDENTIFIER, b"foo", 0..3);

assert_lex!(
    test_tIDENTIFIER_multibyte,
    b"\xD0\xB0\xD0\xB1\xD0\xB2+",
    tIDENTIFIER,
    b"\xD0\xB0\xD0\xB1\xD0\xB2",
    0..6
);

assert_lex!(test_tFID_predicate, b"foo?", tFID, b"foo?", 0..4);
assert_lex!(test_tFID_predicate_eq, b"foo?=", tIDENTIFIER, b"foo", 0..3);
assert_lex!(test_tFID_bang, b"foo!", tFID, b"foo!", 0..4);
assert_lex!(test_tFID_bang_eq, b"foo!=", tIDENTIFIER, b"foo", 0..3);
assert_lex!(test_tIDENTIFIER_setter, b"foo=", tIDENTIFIER, b"foo=", 0..4);
assert_lex!(
    test_tIDENTIFIER_setter_tilde,
    b"foo=~",
    tIDENTIFIER,
    b"foo",
    0..3
);
assert_lex!(
    test_tIDENTIFIER_setter_eq,
    b"foo==",
    tIDENTIFIER,
    b"foo",
    0..3
);
assert_lex!(
    test_tIDENTIFIER_setter_gt,
    b"foo=>",
    tIDENTIFIER,
    b"foo",
    0..3
);

assert_lex!(test_tLABEL, b"foo:", tLABEL, b"foo:", 0..4);

assert_lex!(test_reserved_word, b"if", kIF, b"if", 0..2);
