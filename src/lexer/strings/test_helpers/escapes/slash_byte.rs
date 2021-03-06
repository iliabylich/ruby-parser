macro_rules! assert_emits_escaped_slash_byte {
    (
        literal = $literal:expr
    ) => {
        #[test]
        fn test_escaped_slash_byte() {
            use crate::{
                buffer::BufferWithCursor,
                lexer::strings::{StringExtendAction, StringLiteralExtend},
                loc::loc,
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
                    token: token!(tSTRING_CONTENT, loc!(0, 3))
                })
            );

            action = literal.extend(&mut buffer, 0);
            assert_eq!(
                action,
                ControlFlow::Break(StringExtendAction::EmitToken {
                    token: token!(tSTRING_CONTENT, loc!(3, 5), b'\t')
                })
            );

            action = literal.extend(&mut buffer, 0);
            assert_eq!(
                action,
                ControlFlow::Break(StringExtendAction::EmitToken {
                    token: token!(tSTRING_CONTENT, loc!(5, 8))
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
            token = token!(tSTRING_CONTENT, loc!(0, 8)),
            pre = |_| {}
        );
    };
}
pub(crate) use assert_ignores_escaped_slash_byte;
