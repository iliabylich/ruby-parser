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
            use std::ops::ControlFlow;
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

macro_rules! assert_ignores_escaped_slash_byte {
    (
        literal = $literal:expr
    ) => {
        assert_emits_1_token_and_then_eof!(
            test = test_escaped_slash_byte,
            literal = $literal,
            input = b"foo\\\tbar",
            token = token!(tSTRING_CONTENT(StringContent::from(b"foo\\\tbar")), 0, 8),
            pre = |_| {}
        );
    };
}
pub(crate) use assert_ignores_escaped_slash_byte;
