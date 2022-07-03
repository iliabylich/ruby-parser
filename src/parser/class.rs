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
    pub(crate) fn try_class(&mut self) -> Option<Box<Node<'a>>> {
        // | k_class cpath superclass bodystmt k_end
        // | k_class tLSHFT expr term bodystmt k_end
        todo!("try_class")
    }
}
