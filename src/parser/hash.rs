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
    pub(crate) fn try_hash(&mut self) -> Result<Box<Node>, ParseError> {
        let lcurly_t = self.try_token(TokenKind::tLCURLY)?;
        let assoc_list = self.parse_assoc_list();
        let rcurly_t = self.expect_token(TokenKind::tRCURLY);
        todo!("hash {:?} {:?} {:?}", lcurly_t, assoc_list, rcurly_t);
    }
}
