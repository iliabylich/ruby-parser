macro_rules! assert_emits_interpolated_value {
    ($literal:expr) => {
        // "#{foo}"
        assert_emits_extend_action!(
            test = test_plain_interp,
            literal = $literal,
            input = b"#{TEST_TOKEN",
            action = StringExtendAction::FoundInterpolation {
                token: token!(tSTRING_DBEG, 0, 2)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::ReadInterpolatedContent,
                    "2nd action daction doesn't match"
                )
            }
        );

        // "#@@cvar"
        assert_emits_extend_action!(
            test = test_interp_raw_cvar,
            literal = $literal,
            input = b"#@@cvar",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_DVAR, 0, 1)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitToken {
                        token: token!(tCVAR, 1, 7)
                    },
                    "2nd action daction doesn't match"
                )
            }
        );

        // "#@ivar"
        assert_emits_extend_action!(
            test = test_interp_raw_ivar,
            literal = $literal,
            input = b"#@ivar",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_DVAR, 0, 1)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitToken {
                        token: token!(tIVAR, 1, 6)
                    },
                    "2nd action daction doesn't match"
                )
            }
        );

        // "#$gvar"
        assert_emits_extend_action!(
            test = test_interp_raw_gvar,
            literal = $literal,
            input = b"#$gvar",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_DVAR, 0, 1)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitToken {
                        token: token!(tGVAR, 1, 6)
                    },
                    "2nd action daction doesn't match"
                )
            }
        );

        // "#@@1"
        assert_emits_extend_action!(
            test = test_interp_raw_cvar_invalid,
            literal = $literal,
            input = b"#@@1",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::from(b"#@@1")), 0, 4)
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

        // "#@1"
        assert_emits_extend_action!(
            test = test_interp_raw_ivar_invalid,
            literal = $literal,
            input = b"#@1",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::from(b"#@1")), 0, 3)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitEOF { at: 3 },
                    "2nd action daction doesn't match"
                )
            }
        );

        // "#$("
        assert_emits_extend_action!(
            test = test_interp_raw_gvar_invalid,
            literal = $literal,
            input = b"#$(",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::from(b"#$(")), 0, 3)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitEOF { at: 3 },
                    "2nd action daction doesn't match"
                )
            }
        );

        // "#@@"
        assert_emits_extend_action!(
            test = test_interp_raw_cvar_no_id,
            literal = $literal,
            input = b"#@@",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::from(b"#@@")), 0, 3)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitEOF { at: 3 },
                    "2nd action daction doesn't match"
                )
            }
        );

        // "#@"
        assert_emits_extend_action!(
            test = test_interp_raw_ivar_no_id,
            literal = $literal,
            input = b"#@",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::from(b"#@")), 0, 2)
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

        // "#$ "
        assert_emits_extend_action!(
            test = test_interp_raw_gvar_no_id,
            literal = $literal,
            input = b"#$ ",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT(StringContent::from(b"#$ ")), 0, 3)
            },
            pre = |_| {},
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitEOF { at: 3 },
                    "2nd action daction doesn't match"
                )
            }
        );
    };
}
pub(crate) use assert_emits_interpolated_value;
