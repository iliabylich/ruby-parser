use crate::{
    builder::Constructor,
    parser::{ParseResult, Parser},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_class(&mut self) -> ParseResult<Box<Node>> {
        // | k_class cpath superclass bodystmt k_end
        // | k_class tLSHFT expr term bodystmt k_end
        todo!("try_class")
    }
}
