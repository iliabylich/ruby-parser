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
    pub(crate) fn try_array(&mut self) -> ParseResult<Box<Node>> {
        let lbrack_t = self.try_token(TokenKind::tLBRACK)?;
        let aref_args = self.try_aref_args()?;
        let rbrack_t = self.expect_token(TokenKind::tRBRACK);
        todo!("array {:?} {:?} {:?}", lbrack_t, aref_args, rbrack_t);
    }
}
