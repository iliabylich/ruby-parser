macro_rules! assert_emits_1_token_and_then_eof {
    (
        test = $test:ident,
        literal = $literal:expr,
        input = $input:expr,
        token = $token:expr,
        pre = $pre:expr
    ) => {
        assert_emits_extend_action!(
            test = $test,
            literal = $literal,
            input = $input,
            action = StringExtendAction::EmitToken { token: $token },
            pre = $pre,
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitEOF {
                        at: $token.loc().end
                    },
                    "expected to emit EOF after token"
                )
            }
        );
    };
}
pub(crate) use assert_emits_1_token_and_then_eof;
