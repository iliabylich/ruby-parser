use super::*;

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn parse_preexe(&mut self) -> Option<Box<Node<'a>>> {
        let begin_t = self.try_token(TokenValue::klBEGIN)?;
        let lcurly = self.expect_token(TokenValue::tLCURLY);
        let top_compstmt = self.parse_top_compstmt();
        let rcurly = self.expect_token(TokenValue::tRCURLY);
        panic!(
            "preexe({:?} {:?}, {:?}, {:?})",
            begin_t, lcurly, top_compstmt, rcurly
        )
    }
}
