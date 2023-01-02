use crate::{
    builder::{Builder, LoopType},
    parser::{
        base::{at_most_one_is_true, Repeat1, Rule},
        value::call_tail::CallTail,
        Alias, Args, Array, BackRef, Bodystmt, Case, Class, Compstmt, DoT, EndlessMethodDef,
        ForLoop, Hash, IfStmt, KeywordCmd, Lambda, Literal, MaybeBlock, MethodDef, Module,
        OperationT, ParenArgs, Postexe, Undef, UnlessStmt, Value, VarRef,
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
            EndlessMethodDef::<Value>::starts_now(parser),
            Alias::starts_now(parser),
            Undef::starts_now(parser),
            Postexe::starts_now(parser),
            parser.current_token().is(TokenKind::tFID),
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
        } else if EndlessMethodDef::<Value>::starts_now(parser) {
            EndlessMethodDef::<Value>::parse(parser)
        } else if Alias::starts_now(parser) {
            Alias::parse(parser)
        } else if Undef::starts_now(parser) {
            Undef::parse(parser)
        } else if Postexe::starts_now(parser) {
            Postexe::parse(parser)
        } else if parser.current_token().is(TokenKind::tFID) {
            todo!()
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
                } => todo!(),
                CallTail::ArefArgs {
                    lbrack_t,
                    args,
                    rbrack_t,
                    block,
                } => todo!(),
            }
        }

        head
    }
}
#[test]
fn test_value0_tfid() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Value0, b"foo?", "TODO")
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
        VarRef::starts_now(parser) || OperationT::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if VarRef::starts_now(parser) && OperationT::starts_now(parser) {
            // ambiguity `foo` vs `foo(42)`, depends on the presence of args/curly block
            let name_t = parser.take_token();
            if ParenArgs::starts_now(parser) {
                // `foo(...` method call
                todo!()
            } else if Args::starts_now(parser) && parser.lexer.seen_whitespace {
                // `foo bar ...` command
                todo!()
            } else if let Some(block) = MaybeBlock::parse(parser) {
                // `foo { ...` command
                todo!()
            } else {
                // `foo`/`Foo` variable/const
                match name_t.kind {
                    TokenKind::tIDENTIFIER => Builder::lvar(name_t, parser.buffer()),
                    TokenKind::tCONSTANT => Builder::const_(name_t, parser.buffer()),
                    _ => todo!("{:?}", name_t),
                }
            }
        } else if VarRef::starts_now(parser) {
            VarRef::parse(parser)
        } else if OperationT::starts_now(parser) {
            // method call
            todo!()
        } else {
            unreachable!()
        }
    }
}
#[test]
fn test_method_call_with_open_args() {
    todo!()
}
#[test]
fn test_method_call_with_paren_args() {
    todo!()
}
#[test]
fn test_method_call_with_no_args() {
    todo!()
}

struct Parenthesized;
impl Rule for Parenthesized {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tLPAREN)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        todo!()
    }
}
#[test]
fn test_parenthesized_empty_parens() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Parenthesized, b"()", "TODO")
}
#[test]
fn test_parenthesized_value() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Parenthesized, b"(42)", "TODO")
}
#[test]
fn test_parenthesized_compstmt() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Parenthesized, b"(1; 2; 3)", "TODO")
}

struct Not;
impl Rule for Not {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kNOT)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        todo!()
    }
}
#[test]
fn test_not_parenthesized_value() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Not, b"not(42)", "TODO")
}
#[test]
fn test_not_empty_parens() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Not, b"not()", "TODO")
}
