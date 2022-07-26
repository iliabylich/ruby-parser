use crate::{
    builder::Constructor,
    parser::{ParseError, Parser},
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_defined(&mut self) -> Result<Box<Node>, ParseError> {
        let defined_t = self.try_token(TokenKind::kDEFINED)?;
        let _ = self.try_opt_nl();
        let lparen_t = self.expect_token(TokenKind::tLPAREN);
        let expr = self.try_expr()?;
        let rparen_t = self.expect_token(TokenKind::tRPAREN);
        todo!(
            "defined {:?} {:?} {:?} {:?}",
            defined_t,
            lparen_t,
            expr,
            rparen_t
        )
    }
}
