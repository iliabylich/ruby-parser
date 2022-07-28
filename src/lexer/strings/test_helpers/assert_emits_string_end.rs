macro_rules! assert_emits_string_end {
    (
        literal = $literal:expr,
        begin = $begin:expr,
        end = $end:expr
    ) => {
        assert_emits_extend_action!(
            test = test_string_end,
            literal = $literal,
            input = concat!($end).as_bytes(),
            action = StringExtendAction::FoundStringEnd {
                token: token!(tSTRING_END, loc!(0, 1))
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

        // test that
        //   %Q{ {} }
        // doesn't close literal in the first `}`
        // (i.e. that nested `{}` is interpeted as literal content)
        #[test]
        fn test_string_begin_end_end() {
            use crate::{
                buffer::BufferWithCursor,
                lexer::strings::{StringExtendAction, StringLiteralExtend},
                loc::loc,
                token::token,
            };
            use std::ops::ControlFlow;

            let mut literal = $literal;
            let input = concat!($begin, $end, $end).as_bytes();
            let mut buffer = BufferWithCursor::new(input);
            let mut action;

            action = literal.extend(&mut buffer, 0);
            assert_eq!(
                action,
                ControlFlow::Break(StringExtendAction::EmitToken {
                    token: token!(tSTRING_CONTENT, loc!(0, 2))
                })
            );

            action = literal.extend(&mut buffer, 0);
            assert_eq!(
                action,
                ControlFlow::Break(StringExtendAction::FoundStringEnd {
                    token: token!(tSTRING_END, loc!(2, 3))
                })
            );

            action = literal.extend(&mut buffer, 0);
            assert_eq!(
                action,
                ControlFlow::Break(StringExtendAction::EmitEOF { at: 3 })
            );
        }
    };
}
pub(crate) use assert_emits_string_end;
