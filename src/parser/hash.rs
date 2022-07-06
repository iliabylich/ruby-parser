use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::TokenValue,
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_hash(&mut self) -> Option<Box<Node<'a>>> {
        let lbrace_t = self.try_token(TokenValue::tLCURLY)?;
        let assoc_list = self.parse_assoc_list();
        let rbrace_t = self.expect_token(TokenValue::tRCURLY);
        todo!("hash {:?} {:?} {:?}", lbrace_t, assoc_list, rbrace_t);
    }
}
