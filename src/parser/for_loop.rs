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
    pub(crate) fn try_for_loop(&mut self) -> Option<Box<Node<'a>>> {
        todo!("k_for for_var kIN expr_value_do compstmt k_end")
    }
}
