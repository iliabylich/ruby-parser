use crate::{
    builder::Constructor,
    parser::{ParseResult, Parser},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_for_loop(&mut self) -> ParseResult<Box<Node>> {
        todo!("k_for for_var kIN expr_value_do compstmt k_end")
    }
}
