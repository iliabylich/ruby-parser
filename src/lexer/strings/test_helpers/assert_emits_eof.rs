macro_rules! assert_emits_eof {
    ($literal:expr) => {
        assert_emits_extend_action!(
            test = test_eof,
            literal = $literal,
            input = b"",
            action = StringExtendAction::EmitEOF,
            pre = |_| {},
            post = |_| {}
        );
    };
}
pub(crate) use assert_emits_eof;
