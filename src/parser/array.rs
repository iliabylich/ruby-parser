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
    pub(crate) fn try_array(&mut self) -> Option<Box<Node<'a>>> {
        let lbrack_t = self.try_token(TokenValue::tLBRACK)?;
        let aref_args = self.try_aref_args();
        let rbrack_t = self.expect_token(TokenValue::tRBRACK);
        todo!("array {:?} {:?} {:?}", lbrack_t, aref_args, rbrack_t);
    }
}
