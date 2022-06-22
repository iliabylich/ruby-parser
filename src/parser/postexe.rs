use super::*;

impl<'a> Parser<'a> {
    pub(crate) fn parse_postexe(&mut self) -> Box<Node<'a>> {
        let k_l_end = self.take_token();
        let lcurly = self.expect_token(TokenValue::tLCURLY);
        let compstmt = self.parse_compstmt();
        let rcurly = self.expect_token(TokenValue::tRCURLY);
        panic!(
            "postexe({:?}, {:?}, {:?}, {:?})",
            k_l_end, lcurly, compstmt, rcurly
        )
    }
}
