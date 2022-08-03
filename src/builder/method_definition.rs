use crate::{
    buffer::Buffer,
    builder::{
        helpers::{maybe_boxed_node_expr, string_value},
        Builder,
    },
    nodes::{Alias, Def, Defs, Undef},
    token::Token,
    Node,
};

impl Builder {
    pub(crate) fn def_method(
        def_t: Token,
        name_t: Token,
        args: Option<Box<Node>>,
        body: Option<Box<Node>>,
        end_t: Token,
        buffer: &Buffer,
    ) -> Box<Node> {
        let name_l = name_t.loc;
        let keyword_l = def_t.loc;
        let end_l = end_t.loc;
        let expression_l = keyword_l.join(&end_l);

        let name = string_value(name_l, buffer);
        // self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Box::new(Node::Def(Def {
            name,
            args,
            body,
            keyword_l,
            name_l,
            end_l: Some(end_l),
            assignment_l: None,
            expression_l,
        }))
    }

    pub(crate) fn def_endless_method(
        def_t: Token,
        name_t: Token,
        args: Option<Box<Node>>,
        assignment_t: Token,
        body: Option<Box<Node>>,
        buffer: &Buffer,
    ) -> Box<Node> {
        let body_l = maybe_boxed_node_expr(&body)
            .unwrap_or_else(|| unreachable!("endless method always has a body"));

        let keyword_l = def_t.loc;
        let expression_l = keyword_l.join(&body_l);
        let name_l = name_t.loc;
        let assignment_l = assignment_t.loc;

        let name = string_value(name_l, buffer);
        // self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Box::new(Node::Def(Def {
            name,
            args,
            body,
            keyword_l,
            name_l,
            end_l: None,
            assignment_l: Some(assignment_l),
            expression_l,
        }))
    }

    pub(crate) fn def_singleton(
        def_t: Token,
        definee: Box<Node>,
        dot_t: Token,
        name_t: Token,
        args: Option<Box<Node>>,
        body: Option<Box<Node>>,
        end_t: Token,
        buffer: &Buffer,
    ) -> Box<Node> {
        let keyword_l = def_t.loc;
        let operator_l = dot_t.loc;
        let name_l = name_t.loc;
        let end_l = end_t.loc;
        let expression_l = keyword_l.join(&end_l);

        let name = string_value(name_l, buffer);
        // self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Box::new(Node::Defs(Defs {
            definee,
            name,
            args,
            body,
            keyword_l,
            operator_l,
            name_l,
            assignment_l: None,
            end_l: Some(end_l),
            expression_l,
        }))
    }

    pub(crate) fn def_endless_singleton(
        def_t: Token,
        definee: Box<Node>,
        dot_t: Token,
        name_t: Token,
        args: Option<Box<Node>>,
        assignment_t: Token,
        body: Option<Box<Node>>,
        buffer: &Buffer,
    ) -> Box<Node> {
        let body_l = maybe_boxed_node_expr(&body)
            .unwrap_or_else(|| unreachable!("endless method always has body"));

        let keyword_l = def_t.loc;
        let operator_l = dot_t.loc;
        let name_l = name_t.loc;
        let assignment_l = assignment_t.loc;
        let expression_l = keyword_l.join(&body_l);

        let name = string_value(name_l, buffer);
        // self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Box::new(Node::Defs(Defs {
            definee,
            name,
            args,
            body,
            keyword_l,
            operator_l,
            name_l,
            assignment_l: Some(assignment_l),
            end_l: None,
            expression_l,
        }))
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
