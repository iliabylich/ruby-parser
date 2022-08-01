use crate::{
    builder::{Builder, Constructor},
    nodes::*,
    token::Token,
    Node,
};

impl<C: Constructor> Builder<C> {
    pub(crate) fn logical_op(lhs: Box<Node>, op_t: Token, rhs: Box<Node>) -> Box<Node> {
        // TODO: value_expr(lhs)

        let operator_l = op_t.loc;
        let expression_l = lhs.expression().join(rhs.expression());

        match operator_l.size() {
            2 => {
                // kOR
                Box::new(Node::And(And {
                    lhs,
                    rhs,
                    operator_l,
                    expression_l,
                }))
            }
            3 => {
                // kAND
                Box::new(Node::And(And {
                    lhs,
                    rhs,
                    operator_l,
                    expression_l,
                }))
            }
            _ => unreachable!("only kOR (size = 2) or kAND(size = 3) is supported"),
        }
    }
}
