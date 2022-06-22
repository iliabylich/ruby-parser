use super::*;

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn parse_postexe(&mut self) -> Option<Box<Node<'a>>> {
        let postexe_t = self.try_token(TokenValue::klEND)?;
        let lcurly_t = self.expect_token(TokenValue::tLCURLY);
        let compstmt = self.parse_compstmt();
        let rcurly_t = self.expect_token(TokenValue::tRCURLY);
        Some(Builder::<C>::postexe(
            postexe_t, lcurly_t, compstmt, rcurly_t,
        ))
    }
}

#[test]
fn test_postexe() {
    use crate::{loc::loc, nodes::Postexe, Node, RustParser};
    let mut parser = RustParser::new(b"END {}");
    assert_eq!(
        parser.parse_postexe(),
        Some(Box::new(Node::Postexe(Postexe {
            body: None,
            keyword_l: loc!(0, 3),
            begin_l: loc!(4, 5),
            end_l: loc!(5, 6),
            expression_l: loc!(0, 6)
        })))
    );
}
