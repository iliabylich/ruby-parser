macro_rules! assert_emits_escaped_start_or_end {
    (
        literal = $literal:expr,
        start = $start:literal,
        end = $end:literal
    ) => {
        assert_emits_extend_action!(
            test = test_escaped_start,
            literal = $literal,
            input = concat!("\\", $start).as_bytes(),
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::from($start)), 0, 2)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitEOF,
                    "2nd action daction doesn't match"
                )
            }
        );

        assert_emits_extend_action!(
            test = test_escaped_end,
            literal = $literal,
            input = concat!("\\", $end).as_bytes(),
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::from($end)), 0, 2)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitEOF,
                    "2nd action daction doesn't match"
                )
            }
        );
    };
}
pub(crate) use assert_emits_escaped_start_or_end;
