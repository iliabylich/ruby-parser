use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::TokenKind,
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_hash(&mut self) -> Option<Box<Node<'a>>> {
        let lcurly_t = self.try_token(TokenKind::tLCURLY)?;
        let assoc_list = self.parse_assoc_list();
        let rbrace_t = self.expect_token(TokenKind::tRCURLY);
        todo!("hash {:?} {:?} {:?}", lcurly_t, assoc_list, rbrace_t);
    }
}
