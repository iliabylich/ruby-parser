use crate::{lexer::Lexer, loc::loc, token::token};

#[test]
fn test_string_interp_braces() {
    let mut lexer = Lexer::new(b"\"#{{} + {}}\"");
    assert_eq!(
        lexer.tokenize_until_eof(),
        vec![
            token!(tDSTRING_BEG, loc!(0, 1)),
            token!(tSTRING_DBEG, loc!(1, 3)),
            token!(tLCURLY, loc!(3, 4)),
            token!(tRCURLY, loc!(4, 5)),
            token!(tPLUS, loc!(6, 7)),
            token!(tLCURLY, loc!(8, 9)),
            token!(tRCURLY, loc!(9, 10)),
            token!(tSTRING_DEND, loc!(10, 11)),
            token!(tSTRING_END, loc!(11, 12)),
            token!(tEOF, loc!(12, 12)),
        ]
    );
}
