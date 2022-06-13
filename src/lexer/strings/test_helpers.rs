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
        #[allow(unused_imports)]
        fn $test() {
            use crate::{
                lexer::{
                    buffer::Buffer,
                    string_content::StringContent,
                    strings::{StringExtendAction, StringLiteralExtend},
                },
                token::token,
            };
            use std::borrow::Cow;
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
                match literal {
                    StringLiteral::StringInterp(string) => string.enable_interpolation(),
                    StringLiteral::Regexp(regexp) => regexp.enable_interpolation(),
                    _ => panic!("String literal {:?} doesn't embed Interpolation", literal),
                };
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
                token: token!(tSTRING_CONTENT(StringContent::from(b"#@@1")), 0, 4)
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
                token: token!(tSTRING_CONTENT(StringContent::from(b"#@1")), 0, 3)
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
                token: token!(tSTRING_CONTENT(StringContent::from(b"#$(")), 0, 3)
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
                token: token!(tSTRING_CONTENT(StringContent::from(b"#@@")), 0, 3)
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
                token: token!(tSTRING_CONTENT(StringContent::from(b"#@")), 0, 2)
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
                token: token!(tSTRING_CONTENT(StringContent::from(b"#$ ")), 0, 3)
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
                    StringExtendAction::EmitEOF,
                    "2nd action daction doesn't match"
                )
            }
        );
    };
}
pub(crate) use assert_emits_string_end;

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
                    StringExtendAction::EmitEOF,
                    "2nd action daction doesn't match"
                )
            }
        );
    };
}
pub(crate) use assert_emits_escaped_slash_u;
