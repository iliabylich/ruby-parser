use crate::{Node, Parser, Token};

pub(crate) fn build_prefix_op(op_t: Token, arg: Box<Node>, parser: &mut Parser) -> Box<Node> {
    todo!()
}

pub(crate) fn build_binary_op(
    op_t: Token,
    lhs: Box<Node>,
    parser: &mut Parser,
    r_bp: u8,
) -> Box<Node> {
    todo!()
}

pub(crate) fn build_postfix_op(op_t: Token, arg: Box<Node>, parser: &mut Parser) -> Box<Node> {
    todo!()
}
