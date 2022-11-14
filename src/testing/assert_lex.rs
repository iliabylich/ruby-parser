macro_rules! assert_lex {
    (
        input = $input:expr,
        token = $token:expr,
        setup = $pre:expr,
        assert = $assert:expr
    ) => {{
        use crate::{lexer::Lexer, loc::loc, token::Token};
        let mut lexer = Lexer::new($input);
        $pre(&mut lexer);

        let actual_token = lexer.current_token();
        let expected_token: Token = $token;
        assert_eq!(
            actual_token.kind, expected_token.kind,
            "token doesn't match"
        );
        assert_eq!(actual_token.loc, expected_token.loc, "loc doesn't match");
        assert_eq!(
            actual_token.value, expected_token.value,
            "source of the loc doesn't match"
        );
        assert_eq!(
            actual_token.loc.end,
            lexer.buffer.pos(),
            "buffer.pos() is not token.loc.end (i.e. input hasn't been consumed)"
        );
        $assert(&lexer);
    }};

    // Shortcut with no lexer setup/extra assert
    ($input:expr, $token:expr) => {{
        assert_lex!(
            input = $input,
            token = $token,
            setup = |_lexer: &mut Lexer| {},
            assert = |_lexer: &Lexer| {}
        );
    }};
}
pub(crate) use assert_lex;
