use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::TokenKind,
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_postexe(&mut self) -> Option<Box<Node<'a>>> {
        let postexe_t = self.try_token(TokenKind::klEND)?;
        let lcurly_t = self.expect_token(TokenKind::tLCURLY);
        let compstmt = self.try_compstmt();
        let rcurly_t = self.expect_token(TokenKind::tRCURLY);
        Some(Builder::<C>::postexe(
            postexe_t, lcurly_t, compstmt, rcurly_t,
        ))
    }
}

#[test]
fn test_postexe() {
    use crate::RustParser;
    let mut parser = RustParser::new(b"END { 42 }");
    assert_eq!(parser.try_postexe(), None);
    todo!("implement me");
}
