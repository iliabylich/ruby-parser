macro_rules! assert_emits_escaped_start_or_end {
    (
        literal = $literal:expr,
        start = $start:literal,
        end = $end:literal
    ) => {
        assert_emits_1_token_and_then_eof!(
            test = test_escaped_start,
            literal = $literal,
            input = concat!("\\", $start).as_bytes(),
            token = token!(tSTRING_CONTENT(StringContent::from($start)), 0, 2),
            pre = |_| {}
        );

        assert_emits_1_token_and_then_eof!(
            test = test_escaped_end,
            literal = $literal,
            input = concat!("\\", $end).as_bytes(),
            token = token!(tSTRING_CONTENT(StringContent::from($end)), 0, 2),
            pre = |_| {}
        );
    };
}
pub(crate) use assert_emits_escaped_start_or_end;
