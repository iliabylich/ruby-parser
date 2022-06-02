use crate::{lexer::Lexer, token::token};

#[test]
fn test_string_plain_non_interp() {
    let mut lexer = Lexer::new(b"'foo'");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            token!(tSTRING_BEG, 0, 1),
            token!(tSTRING_CONTENT, 1, 4),
            token!(tSTRING_END, 4, 5),
            token!(tEOF, 5, 5)
        ]
    );
}

#[test]
fn test_string_plain_interp() {
    let mut lexer = Lexer::new(b"\"foo#{TEST_TOKEN}bar\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            token!(tSTRING_BEG, 0, 1),
            token!(tSTRING_CONTENT, 1, 4),
            token!(tSTRING_DBEG, 4, 6),
            token!(tTEST_TOKEN, 6, 16),
            token!(tSTRING_DEND, 16, 17),
            token!(tSTRING_CONTENT, 17, 20),
            token!(tSTRING_END, 20, 21),
            token!(tEOF, 21, 21)
        ]
    );
}

#[test]
fn test_string_interp_braces() {
    let mut lexer = Lexer::new(b"\"#{{} + {}}\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            token!(tSTRING_BEG, 0, 1),
            token!(tSTRING_DBEG, 1, 3),
            token!(tLCURLY, 3, 4),
            token!(tRCURLY, 4, 5),
            token!(tPLUS, 6, 7),
            token!(tLCURLY, 8, 9),
            token!(tRCURLY, 9, 10),
            token!(tSTRING_DEND, 10, 11),
            token!(tSTRING_END, 11, 12),
            token!(tEOF, 12, 12),
        ]
    );
}

#[test]
fn test_string_iterp_raw_cvar() {
    let mut lexer = Lexer::new(b"\"#@@foo\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            token!(tSTRING_BEG, 0, 1),
            token!(tSTRING_DVAR, 1, 2),
            token!(tCVAR, 2, 7),
            token!(tSTRING_END, 7, 8),
            token!(tEOF, 8, 8),
        ]
    );
}

#[test]
fn test_string_iterp_raw_ivar() {
    let mut lexer = Lexer::new(b"\"#@foo\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            token!(tSTRING_BEG, 0, 1),
            token!(tSTRING_DVAR, 1, 2),
            token!(tIVAR, 2, 6),
            token!(tSTRING_END, 6, 7),
            token!(tEOF, 7, 7),
        ]
    );
}

#[test]
fn test_string_iterp_raw_gvar() {
    let mut lexer = Lexer::new(b"\"#$foo\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            token!(tSTRING_BEG, 0, 1),
            token!(tSTRING_DVAR, 1, 2),
            token!(tGVAR, 2, 6),
            token!(tSTRING_END, 6, 7),
            token!(tEOF, 7, 7),
        ]
    );
}

#[test]
fn test_string_interp_raw_cvar_invalid() {
    let mut lexer = Lexer::new(b"\"#@@1\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            token!(tSTRING_BEG, 0, 1),
            token!(tSTRING_CONTENT, 1, 5),
            token!(tSTRING_END, 5, 6),
            token!(tEOF, 6, 6),
        ]
    );
}

#[test]
fn test_string_interp_raw_ivar_invalid() {
    let mut lexer = Lexer::new(b"\"#@1\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            token!(tSTRING_BEG, 0, 1),
            token!(tSTRING_CONTENT, 1, 4),
            token!(tSTRING_END, 4, 5),
            token!(tEOF, 5, 5),
        ]
    );
}

#[test]
fn test_string_interp_raw_cvar_no_id() {
    let mut lexer = Lexer::new(b"\"#@@\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            token!(tSTRING_BEG, 0, 1),
            token!(tSTRING_CONTENT, 1, 4),
            token!(tSTRING_END, 4, 5),
            token!(tEOF, 5, 5),
        ]
    );
}

#[test]
fn test_string_interp_raw_ivar_no_id() {
    let mut lexer = Lexer::new(b"\"#@\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            token!(tSTRING_BEG, 0, 1),
            token!(tSTRING_CONTENT, 1, 3),
            token!(tSTRING_END, 3, 4),
            token!(tEOF, 4, 4),
        ]
    );
}
