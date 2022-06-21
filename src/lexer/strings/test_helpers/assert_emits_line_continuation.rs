macro_rules! assert_emits_line_continuation {
    (
        literal = $literal:expr
    ) => {
        assert_emits_extend_action!(
            test = test_line_continuation,
            literal = $literal,
            input = b"\\\n",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::Borrowed(b"")), 0, 2)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitEOF { at: 2 },
                    "2nd action daction doesn't match"
                )
            }
        );
    };
}
pub(crate) use assert_emits_line_continuation;
