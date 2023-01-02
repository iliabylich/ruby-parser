use crate::{
    builder::Builder,
    nodes::Send,
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
        let name = match CPath::parse(parser) {
            CPath::Const(name) => name,
            CPath::Inheritance {
                name,
                lt_t,
                superclass,
            } => {
                panic!(
                    "Inheritance in module declaration: {:?} {:?} {:?}",
                    name, lt_t, superclass
                )
            }
        };
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
            let (name, lt_t, superclass) = match dbg!(CPath::parse(parser)) {
                CPath::Const(name) => (name, None, None),
                CPath::Inheritance {
                    name,
                    lt_t,
                    superclass,
                } => (name, Some(lt_t), Some(superclass)),
            };
            let body = Bodystmt::parse(parser);
            let end_t = parser.expect_token(TokenKind::kEND);
            Builder::def_class(class_t, name, lt_t, superclass, body, end_t)
        }
    }
}

#[derive(Debug)]
enum CPath {
    Const(Box<Node>),

    Inheritance {
        name: Box<Node>,
        lt_t: Token,
        superclass: Box<Node>,
    },
}
impl Rule for CPath {
    type Output = Self;

    fn starts_now(parser: &mut Parser) -> bool {
        Value::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let node = Value::parse(parser);

        if matches!(&*node, Node::Const(_)) {
            return Self::Const(node);
        }

        match *node {
            Node::Send(Send {
                recv: Some(recv),
                method_name,
                args,
                selector_l: Some(selector_l),
                ..
            }) if method_name.as_bytes() == b"<" && args.len() == 1 => {
                let name = recv;
                let lt_t = Token {
                    kind: TokenKind::tLT,
                    loc: selector_l,
                    value: None,
                };
                let superclass = Box::new(args.into_iter().next().unwrap());
                Self::Inheritance {
                    name,
                    lt_t,
                    superclass,
                }
            }

            other => panic!("Wrong type of CPath: {:?}", other),
        }
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
  s(:const, nil, "Foo"),
  s(:const, nil, "Bar"),
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
