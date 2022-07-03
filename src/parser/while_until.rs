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
    pub(crate) fn try_while_expr(&mut self) -> Option<Box<Node<'a>>> {
        todo!("k_while expr_value_do compstmt k_end")
    }

    pub(crate) fn try_until_expr(&mut self) -> Option<Box<Node<'a>>> {
        todo!("k_until expr_value_do compstmt k_end")
    }
}
