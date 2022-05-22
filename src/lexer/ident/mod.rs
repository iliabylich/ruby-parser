use crate::lexer::buffer::Buffer;
use crate::token::{Loc, Token, TokenValue};

mod reserved_words;

use reserved_words::find_reserved_word;

pub(crate) fn is_identchar(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || !byte.is_ascii()
}

pub(crate) fn parse_ident<'a>(buffer: &mut Buffer<'a>) -> Token<'a> {
    let start = buffer.pos();
    buffer.skip_byte();

    while buffer.current_byte().map(|byte| is_identchar(byte)) == Some(true) {
        buffer.skip_byte();
    }

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

    assert_lex!(test_tIDENTIFIER_plain, "foo", tIDENTIFIER(b"foo"), 0..3);

    assert_lex!(test_tFID_predicate, "foo?", tFID(b"foo?"), 0..4);
    assert_lex!(test_tFID_predicate_eq, "foo?=", tIDENTIFIER(b"foo"), 0..3);
    assert_lex!(test_tFID_bang, "foo!", tFID(b"foo!"), 0..4);
    assert_lex!(test_tFID_bang_eq, "foo!=", tIDENTIFIER(b"foo"), 0..3);
    assert_lex!(test_tIDENTIFIER_setter, "foo=", tIDENTIFIER(b"foo="), 0..4);
    assert_lex!(
        test_tIDENTIFIER_setter_tilde,
        "foo=~",
        tIDENTIFIER(b"foo"),
        0..3
    );
    assert_lex!(
        test_tIDENTIFIER_setter_eq,
        "foo==",
        tIDENTIFIER(b"foo"),
        0..3
    );
    assert_lex!(
        test_tIDENTIFIER_setter_gt,
        "foo=>",
        tIDENTIFIER(b"foo"),
        0..3
    );

    assert_lex!(test_tLABEL, "foo:", tLABEL(b"foo:"), 0..4);

    assert_lex!(test_reserved_word, "if", kIF, 0..2);
}
