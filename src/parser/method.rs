use crate::{
    builder::{Builder, Constructor},
    parser::{ParseError, Parser},
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_method(&mut self) -> Result<Box<Node>, ParseError> {
        // | defn_head f_arglist bodystmt k_end
        // | defs_head f_arglist bodystmt k_end
        todo!("try_method")
    }
}
