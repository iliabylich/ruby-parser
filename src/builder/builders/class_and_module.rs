use crate::{
    builder::{Builder, Constructor},
    Node, Token,
};

impl<C: Constructor> Builder<C> {
    pub(crate) fn def_class(
        class_t: Token,
        name: Box<Node>,
        lt_t: Option<Token>,
        superclass: Option<Box<Node>>,
        body: Option<Box<Node>>,
        end_t: Token,
    ) -> Box<Node> {
        todo!("builder.class")
    }

    pub(crate) fn def_sclass(
        class_t: Token,
        lshift_t: Token,
        expr: Box<Node>,
        body: Option<Box<Node>>,
        end_t: Token,
    ) -> Box<Node> {
        todo!("builder.sclass")
    }

    pub(crate) fn def_module(
        module_t: Token,
        name: Box<Node>,
        body: Option<Box<Node>>,
        end_t: Token,
    ) -> Box<Node> {
        todo!("builder.module")
    }
}
