macro_rules! assert_emits_escaped_slash_u {
    (
        literal = $literal:expr
    ) => {
        assert_emits_extend_action!(
            test = test_escaped_slash_u,
            literal = $literal,
            input = b"\\u1234",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::from("\u{1234}")), 0, 6)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitEOF { at: 6 },
                    "2nd action daction doesn't match"
                )
            }
        );
    };
}
pub(crate) use assert_emits_escaped_slash_u;

macro_rules! assert_ignores_escaped_slash_u {
    (
        literal = $literal:expr
    ) => {
        assert_emits_extend_action!(
            test = test_escaped_slash_u,
            literal = $literal,
            input = b"\\u1234",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::from("\\u1234")), 0, 6)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitEOF { at: 6 },
                    "2nd action daction doesn't match"
                )
            }
        );
    };
}
pub(crate) use assert_ignores_escaped_slash_u;
