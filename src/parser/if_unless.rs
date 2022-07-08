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
    pub(crate) fn try_if_expr(&mut self) -> Option<Box<Node<'a>>> {
        todo!("k_if expr_value then compstmt if_tail k_end")
    }

    pub(crate) fn try_unless_expr(&mut self) -> Option<Box<Node<'a>>> {
        todo!("k_unless expr_value then compstmt opt_else k_end")
    }
}
