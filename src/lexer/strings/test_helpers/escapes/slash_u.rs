macro_rules! assert_emits_escaped_slash_u {
    (
        literal = $literal:expr
    ) => {
        assert_emits_1_token_and_then_eof!(
            test = test_escaped_slash_u,
            literal = $literal,
            input = b"\\u1234",
            token = token!(tSTRING_CONTENT(StringContent::from("\u{1234}")), 0, 6),
            pre = |_| {}
        );
    };
}
pub(crate) use assert_emits_escaped_slash_u;

macro_rules! assert_ignores_escaped_slash_u {
    (
        literal = $literal:expr
    ) => {
        assert_emits_1_token_and_then_eof!(
            test = test_escaped_slash_u,
            literal = $literal,
            input = b"\\u1234",
            token = token!(tSTRING_CONTENT(StringContent::from(b"\\u1234")), 0, 6),
            pre = |_| {}
        );
    };
}
pub(crate) use assert_ignores_escaped_slash_u;
