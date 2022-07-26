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
        let (lbrack_t, elements, rbrack_t) = self
            .all_of("array")
            .and(|| self.try_token(TokenKind::tLBRACK))
            .and(|| self.try_aref_args())
            .and(|| self.expect_token(TokenKind::tRBRACK))
            .stop()?;

        todo!("array {:?} {:?} {:?}", lbrack_t, elements, rbrack_t);
    }
}
