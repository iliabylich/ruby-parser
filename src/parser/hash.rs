use crate::{
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_hash(&mut self) -> ParseResult<Box<Node>> {
        let (lcurly_t, assoc_list, rcurly_t) = self
            .all_of("hash")
            .and(|| self.parse_token(TokenKind::tLCURLY))
            .and(|| self.parse_assoc_list())
            .and(|| self.expect_token(TokenKind::tRCURLY))
            .stop()?;

        todo!("hash {:?} {:?} {:?}", lcurly_t, assoc_list, rcurly_t);
    }
}
