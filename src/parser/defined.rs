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
    pub(crate) fn try_defined(&mut self) -> Option<Box<Node<'a>>> {
        let defined_t = self.try_token(TokenValue::kDEFINED)?;
        let _ = self.try_opt_nl();
        let lparen_t = self.expect_token(TokenValue::tLPAREN);
        if let Some(expr) = self.try_expr() {
            let rparen_t = self.expect_token(TokenValue::tRPAREN);
            todo!(
                "defined {:?} {:?} {:?} {:?}",
                defined_t,
                lparen_t,
                expr,
                rparen_t
            );
        } else {
            panic!("expected expr, got {:?}", self.current_token())
        }
    }
}
