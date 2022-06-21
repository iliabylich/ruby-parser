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

macro_rules! assert_emits_escaped_slash_byte {
    (
        literal = $literal:expr
    ) => {
        #[test]
        fn test_escaped_slash_byte() {
            use crate::{
                lexer::{
                    buffer::BufferWithCursor,
                    string_content::StringContent,
                    strings::{StringExtendAction, StringLiteralExtend},
                },
                token::token,
            };
            let mut buffer = BufferWithCursor::new(b"foo\\\tbar");
            let mut literal = $literal;
            let mut action;

            action = literal.extend(&mut buffer, 0);
            assert_eq!(
                action,
                ControlFlow::Break(StringExtendAction::EmitToken {
                    token: token!(tSTRING_CONTENT(StringContent::from(b"foo")), 0, 3)
                })
            );

            action = literal.extend(&mut buffer, 0);
            assert_eq!(
                action,
                ControlFlow::Break(StringExtendAction::EmitToken {
                    token: token!(tSTRING_CONTENT(StringContent::from(b"\t")), 3, 5)
                })
            );

            action = literal.extend(&mut buffer, 0);
            assert_eq!(
                action,
                ControlFlow::Break(StringExtendAction::EmitToken {
                    token: token!(tSTRING_CONTENT(StringContent::from(b"bar")), 5, 8)
                })
            );

            action = literal.extend(&mut buffer, 0);
            assert_eq!(
                action,
                ControlFlow::Break(StringExtendAction::EmitEOF { at: 8 })
            );
        }
    };
}
pub(crate) use assert_emits_escaped_slash_byte;

macro_rules! assert_emits_escape_sequence {
    (literal = $literal:expr) => {
        assert_emits_escaped_slash_u!(literal = $literal);
        assert_emits_escaped_slash_octal!(literal = $literal);
        assert_emits_escaped_slash_x!(literal = $literal);
        assert_emits_escaped_slash_meta_control!(literal = $literal);
        assert_emits_escaped_slash_byte!(literal = $literal);
    };
}
pub(crate) use assert_emits_escape_sequence;
