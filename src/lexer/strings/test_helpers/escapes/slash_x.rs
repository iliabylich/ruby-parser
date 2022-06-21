macro_rules! assert_emits_escaped_slash_x {
    (
        literal = $literal:expr
    ) => {
        assert_emits_1_token_and_then_eof!(
            test = test_escaped_slash_x,
            literal = $literal,
            input = b"\\x49",
            token = token!(tSTRING_CONTENT(StringContent::from(b"I")), 0, 4),
            pre = |_| {}
        );
    };
}
pub(crate) use assert_emits_escaped_slash_x;

macro_rules! assert_ignores_escaped_slash_x {
    (
        literal = $literal:expr
    ) => {
        assert_emits_1_token_and_then_eof!(
            test = test_escaped_slash_x,
            literal = $literal,
            input = b"\\x49",
            token = token!(tSTRING_CONTENT(StringContent::from("\\x49")), 0, 4),
            pre = |_| {}
        );
    };
}
pub(crate) use assert_ignores_escaped_slash_x;
