macro_rules! assert_emits_escaped_slash_meta_control {
    (
        literal = $literal:expr
    ) => {
        assert_emits_1_token_and_then_eof!(
            test = test_escaped_slash_meta_control,
            literal = $literal,
            input = b"\\C-\\M-a",
            token = token!(tSTRING_CONTENT(StringContent::from([129])), 0, 7),
            pre = |_| {}
        );
    };
}
pub(crate) use assert_emits_escaped_slash_meta_control;

macro_rules! assert_ignores_escaped_slash_meta_control {
    (
        literal = $literal:expr
    ) => {
        assert_emits_1_token_and_then_eof!(
            test = test_escaped_slash_meta_control,
            literal = $literal,
            input = b"\\C-\\M-a",
            token = token!(tSTRING_CONTENT(StringContent::from(b"\\C-\\M-a")), 0, 7),
            pre = |_| {}
        );
    };
}
pub(crate) use assert_ignores_escaped_slash_meta_control;