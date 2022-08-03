use crate::{
    builder::{Builder, Constructor},
    Node, Token,
};

pub(crate) enum KeywordCmd {}

impl<C: Constructor> Builder<C> {
    pub(crate) fn keyword_cmd(
        type_: KeywordCmd,
        keyword_t: Token,
        lparen_t: Option<Token>,
        args: Vec<Node>,
        rparen_t: Option<Token>,
    ) -> Box<Node> {
        todo!("ubuilder.keyword_cmd")
    }
}
