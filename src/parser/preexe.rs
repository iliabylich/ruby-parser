use crate::{parser::base::Rule, Node, Parser, TokenKind};

pub(crate) struct Preexe;
impl Rule for Preexe {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::klBEGIN)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Preexe;
    use crate::testing::assert_parses_rule;

    #[test]
    fn test_preexe() {
        assert_parses_rule!(
            Preexe,
            b"BEGIN { 42 }",
            r#"
s(:preexe,
  s(:int, "42"))
        "#
        );
    }

    #[test]
    fn test_preexe_empty() {
        assert_parses_rule!(Preexe, b"BEGIN {}", "s(:preexe)");
    }
}
