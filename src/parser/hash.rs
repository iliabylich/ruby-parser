use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_hash(&mut self) -> Option<Box<Node>> {
        let lcurly_t = self.try_token(TokenKind::tLCURLY)?;
        let assoc_list = self.parse_assoc_list();
        let rbrace_t = self.expect_token(TokenKind::tRCURLY);
        todo!("hash {:?} {:?} {:?}", lcurly_t, assoc_list, rbrace_t);
    }
}
