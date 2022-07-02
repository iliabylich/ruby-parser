use super::*;

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_postexe(&mut self) -> Option<Box<Node<'a>>> {
        let postexe_t = self.try_token(TokenValue::klEND)?;
        let lcurly_t = self.expect_token(TokenValue::tLCURLY);
        let compstmt = self.try_compstmt();
        let rcurly_t = self.expect_token(TokenValue::tRCURLY);
        Some(Builder::<C>::postexe(
            postexe_t, lcurly_t, compstmt, rcurly_t,
        ))
    }
}

#[test]
fn test_postexe() {
    use crate::RustParser;
    let _parser = RustParser::new(b"END {}");
    unimplemented!("requires parse_primary");
}
