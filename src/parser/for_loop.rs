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
    pub(crate) fn try_for_loop(&mut self) -> Result<Box<Node>, ParseError> {
        todo!("k_for for_var kIN expr_value_do compstmt k_end")
    }
}
