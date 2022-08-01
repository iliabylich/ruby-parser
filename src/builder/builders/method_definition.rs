use crate::{
    builder::{Builder, Constructor},
    nodes::*,
    token::Token,
    Node,
};

impl<C: Constructor> Builder<C> {
    pub(crate) fn def_method() -> Box<Node> {
        todo!("builder.def_method")
    }

    pub(crate) fn def_endless_method() -> Box<Node> {
        todo!("builder.def_endless_method")
    }

    pub(crate) fn def_singleton() -> Box<Node> {
        todo!("builder.def_singleton")
    }

    pub(crate) fn def_endless_singleton() -> Box<Node> {
        todo!("builder.def_endless_singleton")
    }

    pub(crate) fn undef(undef_t: Token, names: Vec<Node>) -> Box<Node> {
        debug_assert!(!names.is_empty());

        let keyword_l = undef_t.loc;
        let expression_l = keyword_l.join(names.last().unwrap().expression());
        Box::new(Node::Undef(Undef {
            names,
            keyword_l,
            expression_l,
        }))
    }

    pub(crate) fn alias(alias_t: Token, to: Box<Node>, from: Box<Node>) -> Box<Node> {
        let keyword_l = alias_t.loc;
        let expression_l = keyword_l.join(from.expression());
        Box::new(Node::Alias(Alias {
            to,
            from,
            keyword_l,
            expression_l,
        }))
    }
}
