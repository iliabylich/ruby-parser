use crate::{
    builder::Builder,
    parser::base::{ParseResult, Rule},
    token::{Token, TokenKind},
    Node, Parser,
};

pub(crate) struct IfStmt;
impl Rule for IfStmt {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kIF)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

pub(crate) struct UnlessStmt;
impl Rule for UnlessStmt {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kUNLESS)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

pub(crate) struct OptElse;

pub(crate) struct Then;

#[cfg(test)]
mod tests {
    use super::{IfStmt, UnlessStmt};
    use crate::testing::assert_parses_rule;

    #[test]
    fn test_if() {
        debug_assert!(false, "implement me");

        assert_parses_rule!(
            IfStmt,
            b"if 1; 2; else; 3; end",
            r#"
s(:if,
  s(:int, "1"),
  s(:int, "2"),
  s(:int, "3"))
            "#
        )
    }

    #[test]
    fn test_unless() {
        debug_assert!(false, "implement me");

        assert_parses_rule!(
            UnlessStmt,
            b"unless 1; 2; else; 3; end",
            r#"
s(:if,
  s(:int, "1"),
  s(:int, "3"),
  s(:int, "2"))
            "#
        )
    }
}
