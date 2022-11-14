use crate::{
    builder::Builder,
    parser::{ParseResult, Rule},
    token::{Token, TokenKind},
    Node, Parser,
};

pub(crate) struct MethodDef;
impl Rule for MethodDef {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

pub(crate) struct EndlessMethodDef<T> {
    _t: std::marker::PhantomData<T>,
}

impl<T> Rule for EndlessMethodDef<T> {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{EndlessMethodDef, MethodDef};
    use crate::testing::assert_parses_rule;

    #[test]
    fn test_instance_method_def() {
        assert_parses_rule!(
            MethodDef,
            b"def foo; 42; end",
            r#"
s(:def, "foo", nil,
  s(:int, "42"))
            "#
        )
    }

    #[test]
    fn test_singleton_method_def() {
        assert_parses_rule!(
            MethodDef,
            b"def self.foo; 42; end",
            r#"
s(:defs,
  s(:self), "foo", nil,
  s(:int, "42"))
            "#
        )
    }

    #[test]
    fn test_singleton_method_def_on_expr() {
        assert_parses_rule!(
            MethodDef,
            b"def (foo).bar; 42; end",
            r#"
s(:defs,
  s(:lvar, "foo"), "bar", nil,
  s(:int, "42"))
            "#
        )
    }
}
