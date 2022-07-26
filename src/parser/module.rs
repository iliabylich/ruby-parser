use crate::{
    builder::Constructor,
    parser::{ParseResult, Parser},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_module(&mut self) -> ParseResult<Box<Node>> {
        todo!("k_module cpath bodystmt k_end")
    }
}
