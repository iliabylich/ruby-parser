use crate::{
    builder::{Builder, LoopType},
    parser::Value,
    Node, Parser, Token, TokenKind,
};

pub(crate) fn build_prefix_op(op_t: Token, arg: Box<Node>, parser: &mut Parser) -> Box<Node> {
    match op_t.kind {
        TokenKind::tDOT2 => Builder::range_inclusive(None, op_t, Some(arg)),
        TokenKind::tDOT3 => Builder::range_exclusive(None, op_t, Some(arg)),

        TokenKind::tMINUS | TokenKind::tPLUS | TokenKind::tTILDE => {
            Builder::unary_op(op_t, arg, parser.buffer())
        }

        TokenKind::tBANG | TokenKind::kNOT => Builder::not_op(op_t, None, Some(arg), None),

        TokenKind::kDEFINED => Builder::defined(op_t, None, arg, None),

        other => unreachable!("{:?}", other),
    }
}
#[test]
fn test_prefix_irange_inc() {
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
#[test]
fn test_prefix_not() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"not true",
        r#"
s(:send,
  s(:true), "!")
        "#
    )
}
#[test]
fn test_prefix_defined() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"defined? true",
        r#"
s(:defined?,
  s(:true))
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
        TokenKind::tPLUS
        | TokenKind::tMINUS
        | TokenKind::tSTAR
        | TokenKind::tDIVIDE
        | TokenKind::tPERCENT
        | TokenKind::tPIPE
        | TokenKind::tCARET
        | TokenKind::tAMPER
        | TokenKind::tCMP
        | TokenKind::tEQ
        | TokenKind::tEQQ
        | TokenKind::tNEQ
        | TokenKind::tNMATCH
        | TokenKind::tMATCH
        | TokenKind::tLSHFT
        | TokenKind::tRSHFT
        | TokenKind::tGT
        | TokenKind::tLT
        | TokenKind::tGEQ
        | TokenKind::tLEQ
        | TokenKind::tDSTAR => Builder::binary_op(lhs, op_t, rhs, parser.buffer()),

        TokenKind::kIF => Builder::condition_mod(Some(lhs), None, op_t, rhs),
        TokenKind::kUNLESS => Builder::condition_mod(None, Some(lhs), op_t, rhs),

        TokenKind::kWHILE => Builder::loop_mod(LoopType::While, lhs, op_t, rhs),
        TokenKind::kUNTIL => Builder::loop_mod(LoopType::Until, lhs, op_t, rhs),

        TokenKind::kRESCUE => {
            let rescue_body = Builder::rescue_body(op_t, vec![], None, None, Some(rhs));
            Builder::begin_body(Some(lhs), vec![*rescue_body], None, None)
        }

        TokenKind::tDOT2 => Builder::range_inclusive(Some(lhs), op_t, Some(rhs)),
        TokenKind::tDOT3 => Builder::range_exclusive(Some(lhs), op_t, Some(rhs)),

        TokenKind::kAND | TokenKind::kOR | TokenKind::tANDOP | TokenKind::tOROP => {
            Builder::logical_op(lhs, op_t, rhs)
        }

        _ => todo!("{:?} {:?} {:?}", lhs, op_t, rhs),
    }
}
#[test]
fn test_binary_op_asgn() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Value, b"true += false", r#""#);
}
#[test]
fn test_binary_asgn() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Value, b"true = false", r#""#);
}
#[test]
fn test_binary_masgn() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Value, b"true, false = nil, __FILE__", r#""#);
}
#[test]
fn test_binary_mlhs_eql_rhs() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Value, b"true, false = nil", r#""#);
}
#[test]
fn test_binary_irange() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true..false",
        r#"
s(:irange,
  s(:true),
  s(:false))
        "#
    );
}
#[test]
fn test_binary_erange() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true...false",
        r#"
s(:erange,
  s(:true),
  s(:false))
        "#
    );
}
#[test]
fn test_binary_plus() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true + false",
        r#"
s(:send,
  s(:true), "+",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_minus() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true - false",
        r#"
s(:send,
  s(:true), "-",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_mul() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true * false",
        r#"
s(:send,
  s(:true), "*",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_div() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true / false",
        r#"
s(:send,
  s(:true), "/",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_mod() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Value, b"true % false", r#""#);
}
#[test]
fn test_binary_pow() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true ** false",
        r#"
s(:send,
  s(:true), "**",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_bin_or() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true | false",
        r#"
s(:send,
  s(:true), "|",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_xor() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true ^ false",
        r#"
s(:send,
  s(:true), "^",
  s(:false))"#
    );
}
#[test]
fn test_binary_bin_and() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true & false",
        r#"
s(:send,
  s(:true), "&",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_cmp() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true <=> false",
        r#"
s(:send,
  s(:true), "<=>",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_eqeq() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true == false",
        r#"
s(:send,
  s(:true), "==",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_eqeqeq() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true === false",
        r#"
s(:send,
  s(:true), "===",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_not_eq() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true != false",
        r#"
s(:send,
  s(:true), "!=",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_match() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true =~ false",
        r#"
s(:send,
  s(:true), "=~",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_nmatch() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true !~ false",
        r#"
s(:send,
  s(:true), "!~",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_lshift() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true << false",
        r#"
s(:send,
  s(:true), "<<",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_rshift() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true >> false",
        r#"
s(:send,
  s(:true), ">>",
  s(:false))
    "#
    );
}
#[test]
fn test_binary_logical_and() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true && false",
        r#"
s(:and,
  s(:true),
  s(:false))
        "#
    );
}
#[test]
fn test_binary_logical_or() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true || false",
        r#"
s(:or,
  s(:true),
  s(:false))
        "#
    );
}
#[test]
fn test_binary_gt() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true > false",
        r#"
s(:send,
  s(:true), ">",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_lt() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true < false",
        r#"
s(:send,
  s(:true), "<",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_gteq() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true >= false",
        r#"
s(:send,
  s(:true), ">=",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_lteq() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true <= false",
        r#"
s(:send,
  s(:true), "<=",
  s(:false))
        "#
    );
}
#[test]
fn test_binary_control_flow_and() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true and false",
        r#"
s(:and,
  s(:true),
  s(:false))
        "#
    );
}
#[test]
fn test_binary_control_flow_or() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true or false",
        r#"
s(:or,
  s(:true),
  s(:false))
      "#
    );
}
#[test]
fn test_binary_ternary_operator() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Value, b"true ? false : nil", r#""#);
}
#[test]
fn test_binary_match_pattern() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Value, b"true => false", r#""#);
}
#[test]
fn test_binary_match_pattern_p() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Value, b"true in false", r#""#);
}
#[test]
fn test_binary_if_mod() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true if false",
        r#"
s(:if,
  s(:false),
  s(:true), nil)
        "#
    );
}
#[test]
fn test_binary_unless_mod() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true unless false",
        r#"
s(:if,
  s(:false), nil,
  s(:true))
        "#
    );
}
#[test]
fn test_binary_while_mod() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true while false",
        r#"
s(:while,
  s(:false),
  s(:true))
        "#
    );
}
#[test]
fn test_binary_until_mod() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true until false",
        r#"
s(:until,
  s(:false),
  s(:true))
        "#
    );
}
#[test]
fn test_binary_rescue_mod() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value,
        b"true rescue false",
        r#"
s(:rescue,
  s(:true),
  s(:resbody, nil, nil,
    s(:false)), nil)
        "#
    );
}

pub(crate) fn build_postfix_op(op_t: Token, arg: Box<Node>, parser: &mut Parser) -> Box<Node> {
    todo!()
}
