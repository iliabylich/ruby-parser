use crate::{
    lexer::Lexer,
    token::{Loc, Token, TokenValue},
};

#[test]
fn test_string_plain_non_interp() {
    let mut lexer = Lexer::new("'foo'");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            Token(TokenValue::tSTRING_BEG(b"'"), Loc(0, 1)),
            Token(TokenValue::tSTRING_CONTENT(b"foo"), Loc(1, 4)),
            Token(TokenValue::tSTRING_END(b"'"), Loc(4, 5)),
            Token(TokenValue::tEOF, Loc(5, 5))
        ]
    );
}

#[test]
fn test_string_plain_interp() {
    let mut lexer = Lexer::new("\"foo#{TEST_TOKEN}bar\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            Token(TokenValue::tSTRING_BEG(b"\""), Loc(0, 1)),
            Token(TokenValue::tSTRING_CONTENT(b"foo"), Loc(1, 4)),
            Token(TokenValue::tSTRING_DBEG(b"#{"), Loc(4, 6)),
            Token(TokenValue::tTEST_TOKEN, Loc(6, 16)),
            Token(TokenValue::tSTRING_DEND, Loc(16, 17)),
            Token(TokenValue::tSTRING_CONTENT(b"bar"), Loc(17, 20)),
            Token(TokenValue::tSTRING_END(b"\""), Loc(20, 21)),
            Token(TokenValue::tEOF, Loc(21, 21))
        ]
    );
}

#[test]
fn test_string_interp_braces() {
    let mut lexer = Lexer::new("\"#{{} + {}}\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            Token(TokenValue::tSTRING_BEG(b"\""), Loc(0, 1)),
            Token(TokenValue::tSTRING_DBEG(b"#{"), Loc(1, 3)),
            Token(TokenValue::tLCURLY, Loc(3, 4)),
            Token(TokenValue::tRCURLY, Loc(4, 5)),
            Token(TokenValue::tPLUS, Loc(6, 7)),
            Token(TokenValue::tLCURLY, Loc(8, 9)),
            Token(TokenValue::tRCURLY, Loc(9, 10)),
            Token(TokenValue::tSTRING_DEND, Loc(10, 11)),
            Token(TokenValue::tSTRING_END(b"\""), Loc(11, 12)),
            Token(TokenValue::tEOF, Loc(12, 12)),
        ]
    );
}
