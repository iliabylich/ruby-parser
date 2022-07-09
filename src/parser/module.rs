use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_module(&mut self) -> Option<Box<Node>> {
        todo!("k_module cpath bodystmt k_end")
    }
}
