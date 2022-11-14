use crate::{
    builder::Builder,
    parser::{ParseResult, Rule},
    token::{Token, TokenKind},
    Node, Parser,
};

pub(crate) struct ForLoop;
impl Rule for ForLoop {
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
    use super::ForLoop;
    use crate::testing::assert_parses_rule;

    #[test]
    fn test_for_lhs() {
        debug_assert!(false, "implement me");

        assert_parses_rule!(
            ForLoop,
            b"for a in 1 do; 2; end",
            r#"
s(:for,
  s(:lvar, "a"),
  s(:int, "1"),
  s(:int, "2"))
            "#
        )
    }

    #[test]
    fn test_for_mlhs() {
        debug_assert!(false, "implement me");

        assert_parses_rule!(
            ForLoop,
            b"for (a, b) in 1 do; 2; end",
            r#"
s(:for,
  s(:begin,
    s(:lvar, "a"),
    s(:lvar, "b")),
  s(:int, "1"),
  s(:int, "2"))
            "#
        )
    }
}
