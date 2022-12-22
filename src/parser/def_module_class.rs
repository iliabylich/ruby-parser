use crate::{
    builder::Builder,
    parser::{base::Rule, Bodystmt, TermT, Value},
    token::{Token, TokenKind},
    Node, Parser,
};

pub(crate) struct Module;
impl Rule for Module {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kMODULE)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let module_t = parser.take_token();
        let name = CPath::parse(parser);
        let body = Bodystmt::parse(parser);
        let end_t = parser.expect_token(TokenKind::kEND);
        Builder::def_module(module_t, name, body, end_t)
    }
}

pub(crate) struct Class;
impl Rule for Class {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kCLASS)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let class_t = parser.take_token();
        if parser.current_token().is(TokenKind::tLSHFT) {
            // class << foo
            let lshift_t = parser.take_token();
            let expr = Value::parse(parser);
            let _ = TermT::parse(parser);
            let body = Bodystmt::parse(parser);
            let end_t = parser.expect_token(TokenKind::kEND);
            Builder::def_sclass(class_t, lshift_t, expr, body, end_t)
        } else {
            let name = CPath::parse(parser);
            let (lt_t, superclass) = Superclass::parse(parser);
            let body = Bodystmt::parse(parser);
            let end_t = parser.expect_token(TokenKind::kEND);
            Builder::def_class(class_t, name, lt_t, superclass, body, end_t)
        }
    }
}

struct Superclass;
impl Rule for Superclass {
    type Output = (Option<Token>, Option<Box<Node>>);

    fn starts_now(_parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if !parser.current_token().is(TokenKind::tLT) {
            return (None, None);
        }
        let lt_t = parser.take_token();
        let superclass = Value::parse(parser);
        let _ = TermT::parse(parser);
        (Some(lt_t), Some(superclass))
    }
}

struct CPath;
impl Rule for CPath {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        Value::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let node = Value::parse(parser);
        // TODO: check that node is Const
        node
    }
}

#[test]
fn test_module() {
    use crate::testing::assert_parses_rule;

    assert_parses_rule!(
        Module,
        b"module Foo::Bar; 1; end",
        r#"
s(:module,
  s(:const,
    s(:const, nil, "Foo"), "Bar"),
  s(:int, "1"))
            "#
    )
}

#[test]
fn test_class() {
    use crate::testing::assert_parses_rule;

    assert_parses_rule!(
        Class,
        b"class Foo < Bar; 1; end",
        r#"
s(:class,
  s(:const,
    s(:const, nil, "Foo"), "Bar"), nil,
  s(:int, "1"))
            "#
    )
}

#[test]
fn test_sclass() {
    use crate::testing::assert_parses_rule;

    assert_parses_rule!(
        Class,
        b"class << self; 1; end",
        r#"
s(:sclass,
  s(:self),
  s(:int, "1"))
            "#
    )
}
