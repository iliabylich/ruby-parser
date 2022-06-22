use super::*;

impl<'a> Parser<'a> {
    pub(crate) fn parse_preexe(&mut self) -> Option<Box<Node<'a>>> {
        if !current_token_is!(self, TokenValue::klBEGIN) {
            return None;
        }
        self.skip_token();

        let lcurly = self.expect_token(TokenValue::tLCURLY);
        let top_compstmt = self.parse_top_compstmt();
        let rcurly = self.expect_token(TokenValue::tRCURLY);
        panic!("preexe({:?}, {:?}, {:?})", lcurly, top_compstmt, rcurly)
    }
}
