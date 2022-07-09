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
    pub(crate) fn try_method(&mut self) -> Option<Box<Node>> {
        // | defn_head f_arglist bodystmt k_end
        // | defs_head f_arglist bodystmt k_end
        todo!("try_method")
    }
}
