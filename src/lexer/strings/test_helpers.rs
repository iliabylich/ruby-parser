macro_rules! assert_emits_extend_action {
    (
        test = $test:ident,
        literal = $literal:expr,
        input = $input:expr,
        action = $action:expr,
        pre = $pre:expr,
        post = $post:expr
    ) => {
        #[test]
        fn $test() {
            #[allow(unused_imports)]
            use crate::{
                lexer::{
                    buffer::Buffer,
                    strings::{StringExtendAction, StringLiteral, StringLiteralExtend},
                },
                token::token,
            };
            let mut literal = $literal;
            let mut buffer = Buffer::new($input);

            $pre(&mut literal);

            let action = literal.extend(&mut buffer, 0);
            assert_eq!(
                action,
                std::ops::ControlFlow::Break($action),
                "1st action doesn't match"
            );

            let action = literal.extend(&mut buffer, 0);
            if let std::ops::ControlFlow::Break(action) = action {
                $post(action);
            } else {
                panic!("2nd action returned Continue which 100% incorrect");
            }
        }
    };
}
pub(crate) use assert_emits_extend_action;

macro_rules! assert_emits_token {
    (
        test = $test:ident,
        literal = $literal:expr,
        input = $input:expr,
        token = $token:expr,
        pre = $pre:expr,
        post = $post:expr
    ) => {
        assert_emits_extend_action!(
            test = $test,
            literal = $literal,
            input = $input,
            action = StringExtendAction::EmitToken { token: $token },
            pre = $pre,
            post = $post
        );
    };
}
pub(crate) use assert_emits_token;

macro_rules! assert_emits_eof_string_action {
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
pub(crate) use assert_emits_eof_string_action;

macro_rules! assert_emits_scheduled_string_action {
    ($literal:expr) => {
        assert_emits_token!(
            test = test_scheduled_action,
            literal = $literal,
            input = b"",
            token = token!(tINTEGER, 0, 1),
            pre = |literal: &mut StringLiteral| {
                literal
                    .inner_mut()
                    .next_action_mut()
                    .add(StringExtendAction::EmitToken {
                        token: token!(tINTEGER, 0, 1),
                    });
            },
            post = |_| {}
        );
    };
}
pub(crate) use assert_emits_scheduled_string_action;

macro_rules! assert_emits_interpolation_end_action {
    ($literal:expr) => {
        assert_emits_extend_action!(
            test = test_interpolation_end,
            literal = $literal,
            input = b"}",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_DEND, 0, 1)
            },
            pre = |literal: &mut StringLiteral| {
                *literal.inner_mut().currently_in_interpolation_mut() = true;
            },
            post = |_| {}
        );
    };
}
pub(crate) use assert_emits_interpolation_end_action;

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
                token: token!(tSTRING_CONTENT, 0, 4)
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

        // "#@1"
        assert_emits_extend_action!(
            test = test_interp_raw_ivar_invalid,
            literal = $literal,
            input = b"#@1",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT, 0, 3)
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

        // "#$("
        assert_emits_extend_action!(
            test = test_interp_raw_gvar_invalid,
            literal = $literal,
            input = b"#$(",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT, 0, 3)
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

        // "#@@"
        assert_emits_extend_action!(
            test = test_interp_raw_cvar_no_id,
            literal = $literal,
            input = b"#@@",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT, 0, 3)
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

        // "#@"
        assert_emits_extend_action!(
            test = test_interp_raw_ivar_no_id,
            literal = $literal,
            input = b"#@",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT, 0, 2)
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

        // "#$ "
        assert_emits_extend_action!(
            test = test_interp_raw_gvar_no_id,
            literal = $literal,
            input = b"#$ ",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_CONTENT, 0, 3)
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
pub(crate) use assert_emits_interpolated_value;

macro_rules! assert_emits_string_end {
    ($literal:expr) => {
        assert_emits_extend_action!(
            test = test_string_end,
            literal = $literal,
            input = b"END",
            action = StringExtendAction::FoundStringEnd {
                token: token!(tSTRING_END, 0, 3)
            },
            pre = |literal: &mut StringLiteral| {
                let inner = literal.inner_mut();
                *inner.ends_with_mut() = b"END";
            },
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
pub(crate) use assert_emits_string_end;
