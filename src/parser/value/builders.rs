use crate::{builder::Builder, parser::Value, Node, Parser, Token, TokenKind};

pub(crate) fn build_prefix_op(op_t: Token, arg: Box<Node>, parser: &mut Parser) -> Box<Node> {
    match op_t.kind {
        TokenKind::tDOT2 => Builder::range_inclusive(None, op_t, Some(arg)),
        TokenKind::tDOT3 => Builder::range_exclusive(None, op_t, Some(arg)),

        TokenKind::tMINUS | TokenKind::tPLUS | TokenKind::tTILDE => {
            Builder::unary_op(op_t, arg, parser.buffer())
        }

        TokenKind::tBANG => Builder::not_op(op_t, None, Some(arg), None),

        other => unreachable!("{:?}", other),
    }
}
#[test]
fn test_prefix_erange_inc() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"..true",
        r#"
s(:irange, nil,
  s(:true))
        "#
    )
}
#[test]
fn test_prefix_erange_exc() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"...true",
        r#"
s(:erange, nil,
  s(:true))
        "#
    )
}
#[test]
fn test_prefix_plus() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"+true",
        r#"
s(:send,
  s(:true), "+@")
        "#
    )
}
#[test]
fn test_prefix_minus() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"-true",
        r#"
s(:send,
  s(:true), "-@")
        "#
    )
}
#[test]
fn test_prefix_bang() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"!true",
        r#"
s(:send,
  s(:true), "!")
        "#
    )
}
#[test]
fn test_prefix_tilde() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"~true",
        r#"
s(:send,
  s(:true), "~")
        "#
    )
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
