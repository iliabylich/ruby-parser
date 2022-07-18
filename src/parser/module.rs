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
    pub(crate) fn try_module(&mut self) -> Result<Box<Node>, ParseError> {
        todo!("k_module cpath bodystmt k_end")
    }
}
