use crate::{
    buffer::{utf8::Utf8Char, Buffer, BufferWithCursor},
    loc::loc,
    token::{token, Token},
};

mod reserved_words;
use reserved_words::find_reserved_word;

mod suffix;
pub(crate) use suffix::IdentSuffix;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Ident {
    pub(crate) length: usize,
}

impl Ident {
    pub(crate) fn lookahead(buffer: &Buffer, start: usize) -> Option<Self> {
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
                Utf8Char::Valid { length } => {
                    // keep consuming
                    end += length;
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
            Some(Ident {
                length: end - start,
            })
        }
    }
}

impl Ident {
    pub(crate) fn is_identchar(byte: u8) -> bool {
        byte.is_ascii_alphanumeric() || byte == b'_' || !byte.is_ascii()
    }

    pub(crate) fn parse(buffer: &mut BufferWithCursor) -> Token {
        let start = buffer.pos();

        let length = match Ident::lookahead(buffer.for_lookahead(), start) {
            Some(Ident { length }) => length,
            None => {
                todo!("handle ident that start with non-UTF-8 byte")
            }
        };

        buffer.set_pos(start + length);

        match IdentSuffix::lookahead(buffer.for_lookahead(), buffer.pos()) {
            Some(IdentSuffix { byte: b'!' | b'?' }) => {
                // append `!` or `?`
                buffer.skip_byte();

                // it still can be a special `defined?` keyword
                if buffer.slice(start, buffer.pos()).unwrap() == b"defined?" {
                    return token!(kDEFINED, loc!(start, buffer.pos()));
                }

                return token!(tFID, loc!(start, buffer.pos()));
            }
            Some(IdentSuffix { byte: b'=' }) => {
                // `foo=` setter, consume `'='
                buffer.skip_byte();
                return token!(tIDENTIFIER, loc!(start, buffer.pos()));
            }
            _ => {}
        }

        let mut const_like = false;
        match buffer.for_lookahead().utf8_char_at(start) {
            Utf8Char::Valid { length } => {
                let s = std::str::from_utf8(buffer.slice(start, start + length).expect("bug"))
                    .expect("bug");
                let c = s.chars().next().expect("bug");
                if c.is_uppercase() {
                    const_like = true;
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
            if const_like && buffer.byte_at(buffer.pos() + 1) == Some(b':') {
                // FOO::BAR case, not a label
            } else {
                buffer.skip_byte();
                return token!(tLABEL, loc!(start, buffer.pos()));
            }
        }

        let end = buffer.pos();
        let slice = buffer.slice(start, end).expect("bug");

        // there's a chance that it's a keyword
        if let Some(reserved_word) = find_reserved_word(slice) {
            return token!(reserved_word.token_value, loc!(start, end));
        }

        // Can be a constant
        if const_like {
            return token!(tCONSTANT, loc!(start, buffer.pos()));
        }

        // otherwise it's just a plain identifier
        token!(tIDENTIFIER, loc!(start, end))
    }
}

#[cfg(test)]
mod tests {
    use super::Ident;
    use crate::{buffer::Buffer, testing::assert_lex, token::token};

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
        assert_eq!(Ident::lookahead(&buffer, 1), Some(Ident { length: 3 })); // captures "foo"

        // valid UTF-8 ("абв")
        let buffer = Buffer::new("абв".as_bytes());
        assert_eq!(Ident::lookahead(&buffer, 0), Some(Ident { length: 6 })); // captures "абв"

        // ASCII ("foo") followed by malformed bytes (208, 0)
        let buffer = Buffer::new(&[b'f', b'o', b'o', 208, 0]);
        assert_eq!(Ident::lookahead(&buffer, 0), Some(Ident { length: 3 })); // captures "foo"

        // UTF-8 ("абв") followed by malformed bytes (208, 0)
        let buffer = Buffer::new(&[208, 176, 208, 177, 208, 177, 208, 0]);
        assert_eq!(Ident::lookahead(&buffer, 0), Some(Ident { length: 6 })); // captures "абв"
    }

    #[test]
    fn test_tIDENTIFIER_plain() {
        assert_lex!(b"foo", token!(tIDENTIFIER, loc!(0, 3)));
    }
    #[test]
    fn test_tCONSTANT_plain() {
        assert_lex!(b"Foo", token!(tCONSTANT, loc!(0, 3)));
    }

    #[test]
    fn test_tIDENTIFIER_multibyte() {
        assert_lex!(
            b"\xD0\xB0\xD0\xB1\xD0\xB2+",
            token!(tIDENTIFIER, loc!(0, 6))
        );
    }
    #[test]
    fn test_tCONSTANT_multibyte() {
        assert_lex!(b"\xD0\x90\xD0\x91\xD0\x92+", token!(tCONSTANT, loc!(0, 6)));
    }

    #[test]
    fn test_tFID_predicate() {
        assert_lex!(b"foo?", token!(tFID, loc!(0, 4)));
    }
    #[test]
    fn test_tFID_predicate_eq() {
        assert_lex!(b"foo?=", token!(tIDENTIFIER, loc!(0, 3)));
    }
    #[test]
    fn test_tFID_bang() {
        assert_lex!(b"foo!", token!(tFID, loc!(0, 4)));
    }
    #[test]
    fn test_tFID_bang_eq() {
        assert_lex!(b"foo!=", token!(tIDENTIFIER, loc!(0, 3)));
    }
    #[test]
    fn test_tIDENTIFIER_setter() {
        assert_lex!(b"foo=", token!(tIDENTIFIER, loc!(0, 4)));
    }
    #[test]
    fn test_tIDENTIFIER_setter_tilde() {
        assert_lex!(b"foo=~", token!(tIDENTIFIER, loc!(0, 3)));
    }
    #[test]
    fn test_tIDENTIFIER_setter_eq() {
        assert_lex!(b"foo==", token!(tIDENTIFIER, loc!(0, 3)));
    }
    #[test]
    fn test_tIDENTIFIER_setter_gt() {
        assert_lex!(b"foo=>", token!(tIDENTIFIER, loc!(0, 3)));
    }

    #[test]
    fn test_tLABEL() {
        assert_lex!(b"foo:", token!(tLABEL, loc!(0, 4)));
    }

    #[test]
    fn test_reserved_word() {
        assert_lex!(b"if", token!(kIF, loc!(0, 2)));
    }
}
