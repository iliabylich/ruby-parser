use crate::{builder::Constructor, parser::Parser, token::Token, Node};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_opt_else(&mut self) -> Option<(Token<'a>, Box<Node<'a>>)> {
        todo!()
    }
}
