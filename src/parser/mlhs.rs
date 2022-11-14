use crate::{
    builder::Builder,
    nodes::{Begin, Node},
    parser::base::{ParseResult, Rule},
    token::TokenKind,
    Parser,
};

pub(crate) struct MLHS;
impl Rule for MLHS {
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
    use super::MLHS;
    use crate::testing::assert_parses_rule;

    #[test]
    fn test_lhs_user_variable() {
        return;

        assert_parses_rule!(
            MLHS,
            b"a, b",
            r#"
s(:begin,
  s(:lvar, "a"),
  s(:lvar, "b"))
            "#
        )
    }

    #[test]
    fn test_lhs_parenthesized() {
        return;

        assert_parses_rule!(
            MLHS,
            b"((a))",
            r#"
s(:begin,
  s(:begin,
    s(:lvar, "a")))
            "#
        )
    }

    #[test]
    fn test_mlhs_without_parens() {
        return;

        assert_parses_rule!(
            MLHS,
            b"a, *b, c",
            r#"
s(:begin,
  s(:lvar, "a"),
  s(:splat,
    s(:lvar, "b")),
  s(:lvar, "c"))
            "#
        )
    }

    #[test]
    fn test_mlhs_with_parens() {
        return;

        assert_parses_rule!(
            MLHS,
            b"((*a), @b, $c)",
            r#"
s(:begin,
  s(:begin,
    s(:splat,
      s(:lvar, "a"))),
  s(:ivar, "@b"),
  s(:gvar, "$c"))
            "#
        );
    }

    #[test]
    fn test_nameless_splat() {
        return;

        assert_parses_rule!(MLHS, b"*", "s(:splat)");
    }
}
