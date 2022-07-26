use crate::{
    builder::Constructor,
    parser::{ParseResult, Parser},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_while_expr(&mut self) -> ParseResult<Box<Node>> {
        todo!("k_while expr_value_do compstmt k_end")
    }

    pub(crate) fn try_until_expr(&mut self) -> ParseResult<Box<Node>> {
        todo!("k_until expr_value_do compstmt k_end")
    }
}
