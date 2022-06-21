macro_rules! assert_emits_escaped_slash_octal {
    (
        literal = $literal:expr
    ) => {
        assert_emits_extend_action!(
            test = test_escaped_slash_octal,
            literal = $literal,
            input = b"\\123",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::from("S")), 0, 4)
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
pub(crate) use assert_emits_escaped_slash_octal;

macro_rules! assert_ignores_escaped_slash_octal {
    (
        literal = $literal:expr
    ) => {
        assert_emits_extend_action!(
            test = test_escaped_slash_octal,
            literal = $literal,
            input = b"\\123",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::from("\\123")), 0, 4)
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
pub(crate) use assert_ignores_escaped_slash_octal;
