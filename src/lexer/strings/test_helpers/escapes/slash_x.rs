macro_rules! assert_emits_escaped_slash_x {
    (
        literal = $literal:expr
    ) => {
        assert_emits_extend_action!(
            test = test_escaped_slash_x,
            literal = $literal,
            input = b"\\x49",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::from("I")), 0, 4)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitEOF { at: 4 },
                    "2nd action daction doesn't match"
                )
            }
        );
    };
}
pub(crate) use assert_emits_escaped_slash_x;

macro_rules! assert_ignores_escaped_slash_x {
    (
        literal = $literal:expr
    ) => {
        assert_emits_extend_action!(
            test = test_escaped_slash_x,
            literal = $literal,
            input = b"\\x49",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::from("\\x49")), 0, 4)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitEOF { at: 4 },
                    "2nd action daction doesn't match"
                )
            }
        );
    };
}
pub(crate) use assert_ignores_escaped_slash_x;
