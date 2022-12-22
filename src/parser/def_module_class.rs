use crate::{
    builder::Builder,
    parser::{base::Rule, Bodystmt, Value},
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
        todo!()
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

#[cfg(test)]
mod tests {
    use super::Module;
    use crate::testing::assert_parses_rule;

    #[test]
    fn test_module() {
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
}
