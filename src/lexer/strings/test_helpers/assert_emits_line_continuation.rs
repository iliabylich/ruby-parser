macro_rules! assert_emits_line_continuation {
    (
        literal = $literal:expr
    ) => {
        assert_emits_1_token_and_then_eof!(
            test = test_line_continuation,
            literal = $literal,
            input = b"\\\n",
            token = token!(tSTRING_CONTENT(StringContent::Borrowed(b"")), 0, 2),
            pre = |_| {}
        );
    };
}
pub(crate) use assert_emits_line_continuation;

macro_rules! assert_ignores_line_continuation {
    (
        literal = $literal:expr
    ) => {
        assert_emits_1_token_and_then_eof!(
            test = test_line_continuation,
            literal = $literal,
            input = b"\\\n",
            token = token!(tSTRING_CONTENT(StringContent::Borrowed(b"\\\n")), 0, 2),
            pre = |_| {}
        );
    };
}
pub(crate) use assert_ignores_line_continuation;
