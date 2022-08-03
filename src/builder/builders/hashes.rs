use crate::{
    builder::{Builder, Constructor},
    Node, Token,
};

impl<C: Constructor> Builder<C> {
    pub(crate) fn pair(key: Box<Node>, assoc_t: Token, value: Box<Node>) -> Box<Node> {
        todo!("builder.pair")
    }

    pub(crate) fn pair_keyword(key_t: Token, value: Box<Node>) -> Box<Node> {
        todo!("builder.pair_keyword")
    }

    pub(crate) fn pair_quoted(
        begin_t: Token,
        parts: Vec<Node>,
        end_t: Token,
        value: Box<Node>,
    ) -> Box<Node> {
        todo!("builder.pair_quoted")
    }

    pub(crate) fn pair_label(key_t: Token) -> Box<Node> {
        todo!("builder.pair_label")
    }

    pub(crate) fn kwsplat(dstar_t: Token, value: Box<Node>) -> Box<Node> {
        todo!("builder.kwsplat")
    }

    pub(crate) fn associate(
        begin_t: Option<Token>,
        pairs: Vec<Node>,
        end_t: Option<Token>,
    ) -> Box<Node> {
        todo!("builder.associate")
    }
}
