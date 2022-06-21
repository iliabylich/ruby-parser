macro_rules! assert_emits_string_end {
    (
        literal = $literal:expr,
        input = $input:expr
    ) => {
        assert_emits_extend_action!(
            test = test_string_end,
            literal = $literal,
            input = $input,
            action = StringExtendAction::FoundStringEnd {
                token: token!(tSTRING_END, 0, 1)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitEOF { at: 1 },
                    "2nd action daction doesn't match"
                )
            }
        );
    };
}
pub(crate) use assert_emits_string_end;
