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
