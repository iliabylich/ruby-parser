use crate::{Lexer, Loc, TokenValue::*};

macro_rules! assert_lex {
    ($test_name:ident, $input:literal, $tok:expr, $loc:expr) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            let mut lexer = Lexer::new($input);
            lexer.tokenize();
            assert_eq!(lexer.tokens[0].value(), $tok);
            assert_eq!(lexer.tokens[0].loc(), Loc($loc.start, $loc.end));
        }
    };
}

// assert_lex!(test_tCOMMENT_INLINE, "# foo", tCOMMENT(b"# foo"), 0..6);

// * block
assert_lex!(test_tSTAR, "*", tSTAR, 0..1);
assert_lex!(test_tOP_ASGN_STAR, "*=", tOP_ASGN(b"*="), 0..2);
assert_lex!(test_tPOW, "**", tPOW, 0..2);
assert_lex!(test_tOP_ASGN_DSTAR, "**=", tOP_ASGN(b"**="), 0..3);

// ! block
assert_lex!(test_tNEQ, "!=", tNEQ, 0..2);
assert_lex!(test_tNMATCH, "!~", tNMATCH, 0..2);
assert_lex!(test_tBANG, "!", tBANG, 0..1);

// = block
assert_lex!(
    test_tEMBEDDED_COMMENT_START,
    "=begin",
    tEMBEDDED_COMMENT_START,
    0..1
);
assert_lex!(test_tEQQ, "===", tEQQ, 0..3);
assert_lex!(test_tEQ, "==", tEQ, 0..2);
assert_lex!(test_tMATCH, "=~", tMATCH, 0..2);
assert_lex!(test_tASSOC, "=>", tASSOC, 0..2);
assert_lex!(test_tEQL, "=", tEQL, 0..1);

// < block

// > block

// " block

// ` block

// ' block

// ? block

// & block

// | block

// + block
assert_lex!(test_tPLUS, "+", tPLUS, 0..1);

// - block
assert_lex!(test_tMINUS, "-", tMINUS, 0..1);

// . block

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
