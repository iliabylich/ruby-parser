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
                    buffer::BufferWithCursor,
                    strings::{StringExtendAction, StringLiteralExtend},
                },
                loc::loc,
                string_content::StringContent,
                token::token,
            };
            let mut literal = $literal;
            let mut buffer = BufferWithCursor::new($input);

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
