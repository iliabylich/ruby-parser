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
    pub(crate) fn try_case(&mut self) -> Option<Box<Node<'a>>> {
        // | k_case expr_value opt_terms case_body k_end
        // | k_case opt_terms case_body k_end
        // | k_case expr_value opt_terms p_case_body k_end
        todo!("try_case")
    }
}