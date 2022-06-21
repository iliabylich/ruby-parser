macro_rules! assert_emits_escaped_slash_octal {
    (
        literal = $literal:expr
    ) => {
        assert_emits_1_token_and_then_eof!(
            test = test_escaped_slash_octal,
            literal = $literal,
            input = b"\\123",
            token = token!(tSTRING_CONTENT(StringContent::from(b"S")), 0, 4),
            pre = |_| {}
        );
    };
}
pub(crate) use assert_emits_escaped_slash_octal;

macro_rules! assert_ignores_escaped_slash_octal {
    (
        literal = $literal:expr
    ) => {
        assert_emits_1_token_and_then_eof!(
            test = test_escaped_slash_octal,
            literal = $literal,
            input = b"\\123",
            token = token!(tSTRING_CONTENT(StringContent::from(b"\\123")), 0, 4),
            pre = |_| {}
        );
    };
}
pub(crate) use assert_ignores_escaped_slash_octal;
