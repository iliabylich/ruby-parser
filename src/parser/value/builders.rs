use crate::{builder::Builder, parser::Value, Node, Parser, Token, TokenKind};

pub(crate) fn build_prefix_op(op_t: Token, arg: Box<Node>, parser: &mut Parser) -> Box<Node> {
    todo!()
}

pub(crate) fn build_binary_op(
    op_t: Token,
    lhs: Box<Node>,
    parser: &mut Parser,
    r_bp: u8,
) -> Box<Node> {
    let rhs = Value::parse_bp(parser, r_bp);

    match op_t.kind {
        TokenKind::tLT | TokenKind::tGT => Builder::binary_op(lhs, op_t, rhs, parser.buffer()),
        _ => todo!("{:?} {:?} {:?}", lhs, op_t, rhs),
    }
}

pub(crate) fn build_postfix_op(op_t: Token, arg: Box<Node>, parser: &mut Parser) -> Box<Node> {
    todo!()
}
