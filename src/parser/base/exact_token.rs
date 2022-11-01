use crate::{
    parser::base::{ParseResult, Rule},
    Parser, Token,
};

pub(crate) struct ExactToken<const TOKEN_KIND: u8>;

impl<const TOKEN_KIND: u8> Rule for ExactToken<TOKEN_KIND> {
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        parser
            .current_token()
            .is(unsafe { std::mem::transmute(TOKEN_KIND) })
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let token = parser.current_token();
        parser.skip_token();
        Ok(token)
    }
}

#[test]
fn test_exact_token() {
    use crate::{loc::loc, token::token, Parser, TokenKind};
    type IntToken = ExactToken<{ TokenKind::tINTEGER as u8 }>;

    let mut parser = Parser::new(b"42");
    assert!(IntToken::starts_now(&mut parser));
    assert_eq!(
        IntToken::parse(&mut parser),
        Ok(token!(tINTEGER, loc!(0, 2)))
    );
}
