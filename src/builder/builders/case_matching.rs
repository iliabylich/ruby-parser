use crate::{
    builder::{Builder, Constructor},
    Node, Token,
};

impl<C: Constructor> Builder<C> {
    pub(crate) fn when(
        when_t: Token,
        patterns: Vec<Node>,
        then_t: Token,
        body: Option<Box<Node>>,
    ) -> Box<Node> {
        todo!("builder.when")
    }

    pub(crate) fn case(
        case_t: Token,
        expr: Option<Box<Node>>,
        when_bodies: Vec<Node>,
        else_t: Option<Token>,
        else_body: Option<Box<Node>>,
        end_t: Token,
    ) -> Box<Node> {
        todo!("builder.case")
    }
}
