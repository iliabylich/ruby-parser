use crate::{
    builder::{Builder, Constructor},
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_postexe(&mut self) -> ParseResult<Box<Node>> {
        let (postexe_t, lcurly_t, compstmt, rcurly_t) = self
            .all_of("postexe")
            .and(|| self.try_token(TokenKind::klEND))
            .and(|| self.expect_token(TokenKind::tLCURLY))
            .and(|| self.try_compstmt())
            .and(|| self.expect_token(TokenKind::tRCURLY))
            .stop()?;

        Ok(Builder::<C>::postexe(
            postexe_t, lcurly_t, compstmt, rcurly_t,
        ))
    }
}

#[test]
fn test_postexe() {
    use crate::{parser::ParseError, RustParser};
    let mut parser = RustParser::new(b"END { 42 }");
    assert_eq!(parser.try_postexe(), Err(ParseError::empty()));
    todo!("implement me");
}
