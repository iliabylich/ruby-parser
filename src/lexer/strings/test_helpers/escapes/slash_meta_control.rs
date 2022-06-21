macro_rules! assert_emits_escaped_slash_meta_control {
    (
        literal = $literal:expr
    ) => {
        assert_emits_extend_action!(
            test = test_escaped_slash_meta_control,
            literal = $literal,
            input = b"\\C-\\M-a",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::from([129])), 0, 7)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitEOF { at: 7 },
                    "2nd action daction doesn't match"
                )
            }
        );
    };
}
pub(crate) use assert_emits_escaped_slash_meta_control;

macro_rules! assert_ignores_escaped_slash_meta_control {
    (
        literal = $literal:expr
    ) => {
        assert_emits_extend_action!(
            test = test_escaped_slash_meta_control,
            literal = $literal,
            input = b"\\C-\\M-a",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::from(b"\\C-\\M-a")), 0, 7)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitEOF { at: 7 },
                    "2nd action daction doesn't match"
                )
            }
        );
    };
}
pub(crate) use assert_ignores_escaped_slash_meta_control;
