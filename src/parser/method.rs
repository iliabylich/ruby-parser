use crate::{
    builder::Constructor,
    parser::{ParseResult, Parser},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_method(&mut self) -> ParseResult<Box<Node>> {
        // | defn_head f_arglist bodystmt k_end
        // | defs_head f_arglist bodystmt k_end
        todo!("try_method")
    }
}
