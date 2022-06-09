use crate::{lexer::Lexer, token::token};

#[test]
fn test_string_interp_braces() {
    let mut lexer = Lexer::new(b"\"#{{} + {}}\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            token!(tDSTRING_BEG, 0, 1),
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
