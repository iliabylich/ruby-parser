use crate::{parser::base::Rule, Node, Parser, TokenKind};

pub(crate) struct Postexe;
impl Rule for Postexe {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::klEND)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Postexe;
    use crate::testing::assert_parses_rule;

    #[test]
    fn test_postexe() {
        assert_parses_rule!(
            Postexe,
            b"END { 42 }",
            r#"
s(:postexe,
  s(:int, "42"))
        "#
        )
    }

    #[test]
    fn test_postexe_empty() {
        assert_parses_rule!(Postexe, b"END {}", "s(:postexe)")
    }
}
