use crate::{
    builder::{helpers::maybe_loc, Builder},
    nodes::{Class, Module, SClass},
    Node, Token,
};

impl Builder {
    pub(crate) fn def_class(
        class_t: Token,
        name: Box<Node>,
        lt_t: Option<Token>,
        superclass: Option<Box<Node>>,
        body: Option<Box<Node>>,
        end_t: Token,
    ) -> Box<Node> {
        let keyword_l = class_t.loc;
        let end_l = end_t.loc;
        let operator_l = maybe_loc(&lt_t);
        let expression_l = keyword_l.join(&end_l);

        Box::new(Node::Class(Class {
            name,
            superclass,
            body,
            keyword_l,
            operator_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn def_sclass(
        class_t: Token,
        lshift_t: Token,
        expr: Box<Node>,
        body: Option<Box<Node>>,
        end_t: Token,
    ) -> Box<Node> {
        let keyword_l = class_t.loc;
        let end_l = end_t.loc;
        let operator_l = lshift_t.loc;
        let expression_l = keyword_l.join(&end_l);

        Box::new(Node::SClass(SClass {
            expr,
            body,
            keyword_l,
            operator_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn def_module(
        module_t: Token,
        name: Box<Node>,
        body: Option<Box<Node>>,
        end_t: Token,
    ) -> Box<Node> {
        let keyword_l = module_t.loc;
        let end_l = end_t.loc;
        let expression_l = keyword_l.join(&end_l);

        Box::new(Node::Module(Module {
            name,
            body,
            keyword_l,
            end_l,
            expression_l,
        }))
    }
}
