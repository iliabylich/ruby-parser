use crate::{
    builder::Builder,
    nodes::{And, Or},
    token::Token,
    Node, TokenKind,
};

impl Builder {
    pub(crate) fn logical_op(lhs: Box<Node>, op_t: Token, rhs: Box<Node>) -> Box<Node> {
        // TODO: value_expr(lhs)

        let operator_l = op_t.loc;
        let expression_l = lhs.expression().join(rhs.expression());

        match op_t.kind {
            TokenKind::kOR | TokenKind::tOROP => Box::new(Node::Or(Or {
                lhs,
                rhs,
                operator_l,
                expression_l,
            })),
            TokenKind::kAND | TokenKind::tANDOP => Box::new(Node::And(And {
                lhs,
                rhs,
                operator_l,
                expression_l,
            })),
            _ => unreachable!(),
        }
    }
}
