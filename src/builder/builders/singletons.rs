use crate::{
    builder::{Builder, Constructor},
    nodes::{False, Nil, True},
    token::Token,
    Node,
};

impl<C: Constructor> Builder<C> {
    // Singletons
    pub(crate) fn nil(nil_t: Token) -> Box<Node> {
        let loc = nil_t.loc;
        Box::new(Node::Nil(Nil { expression_l: loc }))
    }

    pub(crate) fn true_(true_t: Token) -> Box<Node> {
        let loc = true_t.loc;
        Box::new(Node::True(True { expression_l: loc }))
    }

    pub(crate) fn false_(false_t: Token) -> Box<Node> {
        let loc = false_t.loc;
        Box::new(Node::False(False { expression_l: loc }))
    }
}
