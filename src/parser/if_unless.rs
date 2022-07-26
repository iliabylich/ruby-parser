use crate::{
    builder::Constructor,
    parser::{ParseResult, Parser},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_if_expr(&mut self) -> ParseResult<Box<Node>> {
        todo!("k_if expr_value then compstmt if_tail k_end")
    }

    pub(crate) fn try_unless_expr(&mut self) -> ParseResult<Box<Node>> {
        todo!("k_unless expr_value then compstmt opt_else k_end")
    }
}
