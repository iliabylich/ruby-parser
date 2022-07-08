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
    pub(crate) fn try_module(&mut self) -> Option<Box<Node<'a>>> {
        todo!("k_module cpath bodystmt k_end")
    }
}
