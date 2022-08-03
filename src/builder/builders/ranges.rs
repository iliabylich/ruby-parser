use crate::{
    builder::{helpers::maybe_boxed_node_expr, Builder},
    nodes::{Erange, Irange},
    Node, Token,
};

impl Builder {
    pub(crate) fn range_inclusive(
        left: Option<Box<Node>>,
        dot2_t: Token,
        right: Option<Box<Node>>,
    ) -> Box<Node> {
        let operator_l = dot2_t.loc;
        let expression_l = operator_l
            .maybe_join(&maybe_boxed_node_expr(&left))
            .maybe_join(&maybe_boxed_node_expr(&right));

        Box::new(Node::Irange(Irange {
            left,
            right,
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn range_exclusive(
        left: Option<Box<Node>>,
        dot3_t: Token,
        right: Option<Box<Node>>,
    ) -> Box<Node> {
        let operator_l = dot3_t.loc;
        let expression_l = operator_l
            .maybe_join(&maybe_boxed_node_expr(&left))
            .maybe_join(&maybe_boxed_node_expr(&right));

        Box::new(Node::Erange(Erange {
            left,
            right,
            operator_l,
            expression_l,
        }))
    }
}
