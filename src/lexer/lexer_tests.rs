macro_rules! assert_lex {
    ($test_name:ident, $input:literal, $tok:expr, $loc:expr) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            use crate::{Lexer, Loc, TokenValue::*};
            let mut lexer = Lexer::new($input);
            lexer.tokenize_until_eof();
            assert_eq!(lexer.tokens[0].value(), $tok);
            assert_eq!(lexer.tokens[0].loc(), Loc($loc.start, $loc.end));
        }
    };
}

// 0-9 block
assert_lex!(test_tINTEGER, "42", tINTEGER(b"42"), 0..2);

// ) block
assert_lex!(test_tRPAREN, ")", tRPAREN, 0..1);

// ] block
assert_lex!(test_tRBRACK, "]", tRBRACK, 0..1);

// } block
assert_lex!(test_tRCURLY, "}", tRCURLY, 0..1);

// : block

// / block
assert_lex!(test_tDIVIDE, "/", tDIVIDE, 0..1);

// ^ block

// ; block

// , block

// ~ block

// ( block
assert_lex!(test_tLPAREN, "(", tLPAREN, 0..1);

// [ block
assert_lex!(test_tLBRACK, "[", tLBRACK, 0..1);

// { block

// \ block

// % block

// $ block

// @ block

// _ block

// ident block
