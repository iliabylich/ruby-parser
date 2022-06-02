use crate::{
    lexer::Lexer,
    token::{Loc, Token, TokenValue},
};

#[test]
fn test_string_plain_non_interp() {
    let mut lexer = Lexer::new(b"'foo'");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            Token(TokenValue::tSTRING_BEG, Loc(0, 1)),
            Token(TokenValue::tSTRING_CONTENT, Loc(1, 4)),
            Token(TokenValue::tSTRING_END, Loc(4, 5)),
            Token(TokenValue::tEOF, Loc(5, 5))
        ]
    );
}

#[test]
fn test_string_plain_interp() {
    let mut lexer = Lexer::new(b"\"foo#{TEST_TOKEN}bar\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            Token(TokenValue::tSTRING_BEG, Loc(0, 1)),
            Token(TokenValue::tSTRING_CONTENT, Loc(1, 4)),
            Token(TokenValue::tSTRING_DBEG, Loc(4, 6)),
            Token(TokenValue::tTEST_TOKEN, Loc(6, 16)),
            Token(TokenValue::tSTRING_DEND, Loc(16, 17)),
            Token(TokenValue::tSTRING_CONTENT, Loc(17, 20)),
            Token(TokenValue::tSTRING_END, Loc(20, 21)),
            Token(TokenValue::tEOF, Loc(21, 21))
        ]
    );
}

#[test]
fn test_string_interp_braces() {
    let mut lexer = Lexer::new(b"\"#{{} + {}}\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            Token(TokenValue::tSTRING_BEG, Loc(0, 1)),
            Token(TokenValue::tSTRING_DBEG, Loc(1, 3)),
            Token(TokenValue::tLCURLY, Loc(3, 4)),
            Token(TokenValue::tRCURLY, Loc(4, 5)),
            Token(TokenValue::tPLUS, Loc(6, 7)),
            Token(TokenValue::tLCURLY, Loc(8, 9)),
            Token(TokenValue::tRCURLY, Loc(9, 10)),
            Token(TokenValue::tSTRING_DEND, Loc(10, 11)),
            Token(TokenValue::tSTRING_END, Loc(11, 12)),
            Token(TokenValue::tEOF, Loc(12, 12)),
        ]
    );
}

#[test]
fn test_string_iterp_raw_cvar() {
    let mut lexer = Lexer::new(b"\"#@@foo\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            Token(TokenValue::tSTRING_BEG, Loc(0, 1)),
            Token(TokenValue::tSTRING_DVAR, Loc(1, 2)),
            Token(TokenValue::tCVAR, Loc(2, 7)),
            Token(TokenValue::tSTRING_END, Loc(7, 8)),
            Token(TokenValue::tEOF, Loc(8, 8)),
        ]
    );
}

#[test]
fn test_string_iterp_raw_ivar() {
    let mut lexer = Lexer::new(b"\"#@foo\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            Token(TokenValue::tSTRING_BEG, Loc(0, 1)),
            Token(TokenValue::tSTRING_DVAR, Loc(1, 2)),
            Token(TokenValue::tIVAR, Loc(2, 6)),
            Token(TokenValue::tSTRING_END, Loc(6, 7)),
            Token(TokenValue::tEOF, Loc(7, 7)),
        ]
    );
}

#[test]
fn test_string_iterp_raw_gvar() {
    let mut lexer = Lexer::new(b"\"#$foo\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            Token(TokenValue::tSTRING_BEG, Loc(0, 1)),
            Token(TokenValue::tSTRING_DVAR, Loc(1, 2)),
            Token(TokenValue::tGVAR, Loc(2, 6)),
            Token(TokenValue::tSTRING_END, Loc(6, 7)),
            Token(TokenValue::tEOF, Loc(7, 7)),
        ]
    );
}

#[test]
fn test_string_interp_raw_cvar_invalid() {
    let mut lexer = Lexer::new(b"\"#@@1\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            Token(TokenValue::tSTRING_BEG, Loc(0, 1)),
            Token(TokenValue::tSTRING_CONTENT, Loc(1, 5)),
            Token(TokenValue::tSTRING_END, Loc(5, 6)),
            Token(TokenValue::tEOF, Loc(6, 6)),
        ]
    );
}

#[test]
fn test_string_interp_raw_ivar_invalid() {
    let mut lexer = Lexer::new(b"\"#@1\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            Token(TokenValue::tSTRING_BEG, Loc(0, 1)),
            Token(TokenValue::tSTRING_CONTENT, Loc(1, 4)),
            Token(TokenValue::tSTRING_END, Loc(4, 5)),
            Token(TokenValue::tEOF, Loc(5, 5)),
        ]
    );
}

#[test]
fn test_string_interp_raw_cvar_no_id() {
    let mut lexer = Lexer::new(b"\"#@@\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            Token(TokenValue::tSTRING_BEG, Loc(0, 1)),
            Token(TokenValue::tSTRING_CONTENT, Loc(1, 4)),
            Token(TokenValue::tSTRING_END, Loc(4, 5)),
            Token(TokenValue::tEOF, Loc(5, 5)),
        ]
    );
}

#[test]
fn test_string_interp_raw_ivar_no_id() {
    let mut lexer = Lexer::new(b"\"#@\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            Token(TokenValue::tSTRING_BEG, Loc(0, 1)),
            Token(TokenValue::tSTRING_CONTENT, Loc(1, 3)),
            Token(TokenValue::tSTRING_END, Loc(3, 4)),
            Token(TokenValue::tEOF, Loc(4, 4)),
        ]
    );
}
