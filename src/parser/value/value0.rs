use crate::{
    builder::{ArgsType, Builder, LoopType},
    parser::{
        base::{at_most_one_is_true, Maybe1, Repeat1, Rule},
        value::call_tail::CallTail,
        Alias, Array, BackRef, Bodystmt, CallArgs, Case, Class, Compstmt, Cvar, DoT,
        EndlessMethodDef, ForLoop, Gvar, Hash, IfStmt, Ivar, KeywordCmd, KeywordVariable, Lambda,
        Literal, MaybeBlock, MethodDef, Module, Postexe, Undef, UnlessStmt, Value,
    },
    Node, Parser, TokenKind,
};

pub(crate) struct Value0;
impl Rule for Value0 {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        at_most_one_is_true([
            Literal::starts_now(parser),
            VarRefOrMethodCall::starts_now(parser),
            Array::starts_now(parser),
            Hash::starts_now(parser),
            BackRef::starts_now(parser),
            Parenthesized::starts_now(parser),
            Not::starts_now(parser),
            Lambda::starts_now(parser),
            IfStmt::starts_now(parser),
            UnlessStmt::starts_now(parser),
            Case::starts_now(parser),
            ForLoop::starts_now(parser),
            Class::starts_now(parser),
            Module::starts_now(parser),
            MethodDef::starts_now(parser),
            KeywordCmd::starts_now(parser),
            EndlessMethodDef::starts_now(parser),
            Alias::starts_now(parser),
            Undef::starts_now(parser),
            Postexe::starts_now(parser),
            parser.current_token().is(TokenKind::kBEGIN),
            parser.current_token().is(TokenKind::tCOLON2),
            parser.current_token().is(TokenKind::kWHILE),
            parser.current_token().is(TokenKind::kUNTIL),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let mut head = if Literal::starts_now(parser) {
            Literal::parse(parser)
        } else if VarRefOrMethodCall::starts_now(parser) {
            VarRefOrMethodCall::parse(parser)
        } else if Array::starts_now(parser) {
            Array::parse(parser)
        } else if Hash::starts_now(parser) {
            Hash::parse(parser)
        } else if BackRef::starts_now(parser) {
            BackRef::parse(parser)
        } else if Parenthesized::starts_now(parser) {
            Parenthesized::parse(parser)
        } else if Not::starts_now(parser) {
            Not::parse(parser)
        } else if Lambda::starts_now(parser) {
            Lambda::parse(parser)
        } else if IfStmt::starts_now(parser) {
            IfStmt::parse(parser)
        } else if UnlessStmt::starts_now(parser) {
            UnlessStmt::parse(parser)
        } else if Case::starts_now(parser) {
            Case::parse(parser)
        } else if ForLoop::starts_now(parser) {
            ForLoop::parse(parser)
        } else if Class::starts_now(parser) {
            Class::parse(parser)
        } else if Module::starts_now(parser) {
            Module::parse(parser)
        } else if MethodDef::starts_now(parser) {
            MethodDef::parse(parser)
        } else if KeywordCmd::starts_now(parser) {
            KeywordCmd::parse(parser)
        } else if EndlessMethodDef::starts_now(parser) {
            EndlessMethodDef::parse(parser)
        } else if Alias::starts_now(parser) {
            Alias::parse(parser)
        } else if Undef::starts_now(parser) {
            Undef::parse(parser)
        } else if Postexe::starts_now(parser) {
            Postexe::parse(parser)
        } else if parser.current_token().is(TokenKind::kBEGIN) {
            let begin_t = parser.take_token();
            let body = Bodystmt::parse(parser);
            let statements = if let Some(body) = body {
                vec![*body]
            } else {
                vec![]
            };
            let end_t = parser.expect_token(TokenKind::kEND);
            Builder::begin(begin_t, statements, end_t)
        } else if parser.current_token().is(TokenKind::tCOLON2) {
            let colon2_t = parser.take_token();
            let name_t = parser.expect_token(TokenKind::tCONSTANT);
            Builder::const_global(colon2_t, name_t, parser.buffer())
        } else if parser.current_token().is(TokenKind::kWHILE) {
            let keyword_t = parser.take_token();
            let cond = Value::parse(parser);
            let do_t = DoT::parse(parser);
            let body = Compstmt::parse(parser);
            let end_t = parser.expect_token(TokenKind::kEND);
            Builder::loop_(LoopType::While, keyword_t, cond, do_t, body, end_t)
        } else if parser.current_token().is(TokenKind::kUNTIL) {
            let keyword_t = parser.take_token();
            let cond = Value::parse(parser);
            let do_t = DoT::parse(parser);
            let body = Compstmt::parse(parser);
            let end_t = parser.expect_token(TokenKind::kEND);
            Builder::loop_(LoopType::Until, keyword_t, cond, do_t, body, end_t)
        } else {
            unreachable!()
        };

        for tail in Repeat1::<CallTail>::parse(parser).into_iter() {
            match tail {
                CallTail::ConstAccess { colon2_t, name_t } => {
                    head = Builder::const_fetch(head, colon2_t, name_t, parser.buffer());
                }
                CallTail::MethodCall {
                    dot_t,
                    name_t,
                    lparen_t,
                    args,
                    rparen_t,
                    block,
                } => {
                    head = Builder::call_method(
                        Some(head),
                        Some(dot_t),
                        name_t,
                        lparen_t,
                        args,
                        rparen_t,
                        parser.buffer(),
                    );

                    if let Some((begin_t, args, body, end_t)) = block {
                        head =
                            Builder::block(head, begin_t, ArgsType::Args(args), Some(body), end_t);
                    }
                }
                CallTail::ArefArgs {
                    lbrack_t,
                    args,
                    rbrack_t,
                    block,
                } => {
                    head = Builder::index(head, lbrack_t, args, rbrack_t);

                    if let Some((begin_t, args, body, end_t)) = block {
                        head =
                            Builder::block(head, begin_t, ArgsType::Args(args), Some(body), end_t);
                    }
                }
            }
        }

        head
    }
}
#[test]
fn test_value0_begin_bodystmt_end() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Value0, b"begin; 42; end", "s(:begin,\n  s(:int, \"42\"))")
}
#[test]
fn test_value0_global_const() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value0,
        b"::FOO",
        r#"
s(:const,
  s(:cbase), "FOO")
        "#
    )
}
#[test]
fn test_value0_while_loop() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value0,
        b"while true; 42; end",
        r#"
s(:while,
  s(:true),
  s(:int, "42"))
        "#
    )
}
#[test]
fn test_value0_until_loop() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value0,
        b"until true; 42; end",
        r#"
s(:until,
  s(:true),
  s(:int, "42"))
        "#
    )
}

struct VarRefOrMethodCall;
// This rule encapsulates variables, constants, methods calls
impl Rule for VarRefOrMethodCall {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        let token = parser.current_token();

        at_most_one_is_true([
            token.is(TokenKind::tIDENTIFIER),
            token.is(TokenKind::tCONSTANT),
            token.is(TokenKind::tFID),
            Ivar::starts_now(parser),
            Gvar::starts_now(parser),
            Cvar::starts_now(parser),
            KeywordVariable::starts_now(parser),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if Ivar::starts_now(parser) {
            Ivar::parse(parser)
        } else if Gvar::starts_now(parser) {
            Gvar::parse(parser)
        } else if Cvar::starts_now(parser) {
            Cvar::parse(parser)
        } else if KeywordVariable::starts_now(parser) {
            KeywordVariable::parse(parser)
        } else {
            // tIDENTIFIER/tCONSTANT/tFID
            let name_t = parser.take_token();
            let (lparen_t, args, rparen_t) = CallArgs::parse(parser);
            let block = MaybeBlock::parse(parser);
            if lparen_t.is_some() || !args.is_empty() || rparen_t.is_some() || block.is_some() {
                // method call with args/block/both
                let mut node = Builder::call_method(
                    None,
                    None,
                    Some(name_t),
                    lparen_t,
                    args,
                    rparen_t,
                    parser.buffer(),
                );
                if let Some((begin_t, args, body, end_t)) = block {
                    node = Builder::block(node, begin_t, ArgsType::Args(args), Some(body), end_t)
                }
                return node;
            }

            match name_t.kind {
                TokenKind::tIDENTIFIER | TokenKind::tFID => {
                    // TODO: check for local variable
                    Builder::call_method(
                        None,
                        None,
                        Some(name_t),
                        None,
                        vec![],
                        None,
                        parser.buffer(),
                    )
                }

                TokenKind::tCONSTANT => Builder::const_(name_t, parser.buffer()),

                _ => unreachable!(),
            }
        }
    }
}
#[test]
fn test_method_call_with_open_args() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value0,
        b"foo 42",
        r#"
s(:send, nil, "foo",
  s(:int, "42"))
        "#
    )
}
#[test]
fn test_method_call_with_paren_args() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Value0,
        b"foo(42)",
        r#"
s(:send, nil, "foo",
  s(:begin,
    s(:int, "42")))
        "#
    )
}
#[test]
fn test_method_call_with_no_args() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Value0, b"foo", "s(:send, nil, \"foo\")")
}
#[test]
fn test_value0_tfid() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Value0, b"foo?", "s(:send, nil, \"foo?\")")
}

struct Parenthesized;
impl Rule for Parenthesized {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tLPAREN)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let begin_t = parser.take_token();
        let body = Compstmt::parse(parser);
        let statements = if let Some(body) = body {
            vec![*body]
        } else {
            vec![]
        };
        let end_t = parser.expect_token(TokenKind::tRPAREN);
        Builder::begin(begin_t, statements, end_t)
    }
}
#[test]
fn test_parenthesized_empty_parens() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Parenthesized, b"()", "s(:begin)")
}
#[test]
fn test_parenthesized_value() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Parenthesized,
        b"(42)",
        r#"
s(:begin,
  s(:int, "42"))
    "#
    )
}
#[test]
fn test_parenthesized_compstmt() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Parenthesized,
        b"(1; 2; 3)",
        r#"
s(:begin,
  s(:begin,
    s(:int, "1"),
    s(:int, "2"),
    s(:int, "3")))
    "#
    )
}

struct Not;
impl Rule for Not {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kNOT) && parser.lexer.lookahead_is_lparen()
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let not_t = parser.take_token();
        let begin_t = parser.expect_token(TokenKind::tLPAREN);
        let receiver = Maybe1::<Value>::parse(parser);
        let end_t = parser.expect_token(TokenKind::tRPAREN);
        Builder::not_op(not_t, Some(begin_t), receiver, Some(end_t))
    }
}
#[test]
fn test_not_parenthesized_value() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Not,
        b"not(42)",
        r#"
s(:send,
  s(:int, "42"), "!")
        "#
    )
}
#[test]
fn test_not_empty_parens() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Not,
        b"not()",
        r#"
s(:send,
  s(:begin), "!")
        "#
    )
}
