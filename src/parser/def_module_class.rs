use crate::{
    builder::Builder,
    parser::base::Rule,
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
        todo!()
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
