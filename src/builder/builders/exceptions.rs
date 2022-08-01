use crate::{
    builder::{Builder, Constructor},
    token::Token,
    Node,
};

impl<C: Constructor> Builder<C> {
    pub(crate) fn rescue_body(
        rescue_t: Token,
        exc_list: Vec<Node>,
        exc_var: Option<(Token, Box<Node>)>,
        then_t: Option<Token>,
        body: Option<Box<Node>>,
    ) -> Box<Node> {
        let exc_list = Self::array(None, exc_list, None);
        todo!("builder.rescue_body")
    }

    pub(crate) fn begin_body(
        compound_stmt: Option<Box<Node>>,
        rescue_bodies: Vec<Node>,
        opt_else: Option<(Token, Option<Box<Node>>)>,
        opt_ensure: Option<(Token, Option<Box<Node>>)>,
    ) -> Box<Node> {
        todo!("builder.begin_body")
    }
}
