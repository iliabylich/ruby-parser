use crate::{
    parser::{
        base::{ParseResult, Rule},
        Alias, Array, BackRef, Case, Class, EndlessMethodDef, ForLoop, Hash, IfStmt, KeywordCmd,
        Lambda, Literal, MethodDef, Module, OperationT, Postexe, Undef, UnlessStmt, Value, VarRef,
    },
    Node, Parser, TokenKind,
};

pub(crate) struct Value0;
impl Rule for Value0 {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        MethodCall::starts_now(parser)
            || Literal::starts_now(parser)
            || Array::starts_now(parser)
            || Hash::starts_now(parser)
            || VarRef::starts_now(parser)
            || BackRef::starts_now(parser)
            || Parenthesized::starts_now(parser)
            || Not::starts_now(parser)
            || Lambda::starts_now(parser)
            || IfStmt::starts_now(parser)
            || UnlessStmt::starts_now(parser)
            || Case::starts_now(parser)
            || ForLoop::starts_now(parser)
            || Class::starts_now(parser)
            || Module::starts_now(parser)
            || MethodDef::starts_now(parser)
            || KeywordCmd::starts_now(parser)
            || EndlessMethodDef::<Value>::starts_now(parser)
            || Alias::starts_now(parser)
            || Undef::starts_now(parser)
            || Postexe::starts_now(parser)
            || parser.current_token().is(TokenKind::tFID)
            || parser.current_token().is(TokenKind::kBEGIN)
            || parser.current_token().is(TokenKind::tCOLON2)
            || parser.current_token().is(TokenKind::kWHILE)
            || parser.current_token().is(TokenKind::kUNTIL)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if MethodCall::starts_now(parser) {
            MethodCall::parse(parser)
        } else if Literal::starts_now(parser) {
            Literal::parse(parser)
        } else if Array::starts_now(parser) {
            Array::parse(parser)
        } else if Hash::starts_now(parser) {
            Hash::parse(parser)
        } else if VarRef::starts_now(parser) {
            VarRef::parse(parser)
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
            todo!()
        } else if parser.current_token().is(TokenKind::tCOLON2) {
            todo!()
        } else if parser.current_token().is(TokenKind::kWHILE) {
            todo!()
        } else if parser.current_token().is(TokenKind::kUNTIL) {
            todo!()
        } else {
            unreachable!()
        }
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
    assert_parses_rule!(Value0, b"begin; 42; end", "TODO")
}
#[test]
fn test_value0_global_const() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Value0, b"::FOO", "TODO")
}
#[test]
fn test_value0_while_loop() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Value0, b"while true; 42; end", "TODO")
}
#[test]
fn test_value0_until_loop() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Value0, b"unless true; 42; end", "TODO")
}

struct MethodCall;
impl Rule for MethodCall {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        OperationT::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
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

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
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

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
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
