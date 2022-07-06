use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::TokenValue,
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_preexe(&mut self) -> Option<Box<Node<'a>>> {
        let begin_t = self.try_token(TokenValue::klBEGIN)?;
        let lcurly = self.expect_token(TokenValue::tLCURLY);
        let top_compstmt = self.try_top_compstmt();
        let rcurly = self.expect_token(TokenValue::tRCURLY);
        Some(Builder::<C>::preexe(begin_t, lcurly, top_compstmt, rcurly))
    }
}

#[test]
fn test_preexe() {
    use crate::parser::RustParser;
    let mut parser = RustParser::new(b"BEGIN {}");
    assert_eq!(parser.try_preexe(), None);
    todo!("implement me");
}

#[test]
fn test_nothing() {
    use crate::parser::RustParser;
    let mut parser = RustParser::new(b"");
    assert_eq!(parser.try_preexe(), None);
}
