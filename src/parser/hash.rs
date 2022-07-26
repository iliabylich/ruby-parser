use crate::{
    builder::Constructor,
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_hash(&mut self) -> ParseResult<Box<Node>> {
        let (lcurly_t, assoc_list, rcurly_t) = self
            .all_of("hash")
            .and(|| self.try_token(TokenKind::tLCURLY))
            .and(|| self.try_assoc_list())
            .and(|| self.expect_token(TokenKind::tRCURLY))
            .stop()?;

        todo!("hash {:?} {:?} {:?}", lcurly_t, assoc_list, rcurly_t);
    }
}
