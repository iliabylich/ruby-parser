use crate::{
    builder::Constructor,
    parser::{ParseResult, Parser},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_case(&mut self) -> ParseResult<Box<Node>> {
        // | k_case expr_value opt_terms case_body k_end
        // | k_case opt_terms case_body k_end
        // | k_case expr_value opt_terms p_case_body k_end
        todo!("try_case")
    }
}
