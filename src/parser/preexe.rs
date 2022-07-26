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
    pub(crate) fn try_preexe(&mut self) -> ParseResult<Box<Node>> {
        let (preexe_t, lcurly_t, body, rcurly_t) = self
            .all_of("preexe")
            .and(|| self.try_token(TokenKind::klBEGIN))
            .and(|| self.expect_token(TokenKind::tLCURLY))
            .and(|| self.try_top_compstmt())
            .and(|| self.expect_token(TokenKind::tRCURLY))
            .stop()?;

        Ok(Builder::<C>::preexe(preexe_t, lcurly_t, body, rcurly_t))
    }
}

#[test]
fn test_preexe() {
    use crate::parser::{ParseError, RustParser};
    let mut parser = RustParser::new(b"BEGIN {}").debug();
    assert_eq!(parser.try_preexe(), Err(ParseError::empty()));
    todo!("implement me");
}

#[test]
fn test_nothing() {
    use crate::{parser::RustParser, transactions::assert_err_eq};
    let mut parser = RustParser::new(b"");
    assert_err_eq!(
        parser.try_postexe(),
        "
SEQUENCE (1) postexe (got [])
    TOKEN (1) expected klEND, got tEOF (at 0)
    "
    );
}
