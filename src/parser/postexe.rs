use crate::{
    builder::{Builder, Constructor},
    parser::{ParseError, Parser},
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_postexe(&mut self) -> Result<Box<Node>, ParseError> {
        let postexe_t = self.try_token(TokenKind::klEND)?;
        let lcurly_t = self.expect_token(TokenKind::tLCURLY);
        let compstmt = self.try_compstmt()?;
        let rcurly_t = self.expect_token(TokenKind::tRCURLY);
        Ok(Builder::<C>::postexe(
            postexe_t, lcurly_t, compstmt, rcurly_t,
        ))
    }
}

#[test]
fn test_postexe() {
    use crate::RustParser;
    let mut parser = RustParser::new(b"END { 42 }");
    assert_eq!(parser.try_postexe(), Err(ParseError::empty()));
    todo!("implement me");
}
