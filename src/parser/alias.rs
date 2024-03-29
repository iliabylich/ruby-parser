use crate::{
    builder::Builder,
    parser::{base::Rule, BackRef, Fitem, Gvar},
    token::TokenKind,
    Node, Parser,
};

pub(crate) struct Alias;
impl Rule for Alias {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kALIAS)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let alias_t = parser.take_token();

        let (lhs, rhs) = if Fitem::starts_now(parser) {
            let lhs = Fitem::parse(parser);
            let rhs = Fitem::parse(parser);
            (lhs, rhs)
        } else if parser.current_token().is(TokenKind::tGVAR) {
            let lhs = Gvar::parse(parser);

            let rhs = if Gvar::starts_now(parser) {
                Gvar::parse(parser)
            } else if BackRef::starts_now(parser) {
                BackRef::parse(parser)
            } else {
                panic!("wring token type")
            };

            (lhs, rhs)
        } else {
            panic!("wrong token type")
        };

        Builder::alias(alias_t, lhs, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::Alias;
    use crate::testing::assert_parses_rule;

    #[test]
    fn test_alias_name_to_name() {
        assert_parses_rule!(
            Alias,
            b"alias foo bar",
            r#"
s(:alias,
  s(:sym, "foo"),
  s(:sym, "bar"))
            "#
        )
    }

    #[test]
    fn test_alias_sym_to_sym() {
        assert_parses_rule!(
            Alias,
            b"alias :foo :bar",
            r#"
s(:alias,
  s(:sym, "foo"),
  s(:sym, "bar"))
            "#
        )
    }

    #[test]
    fn test_alias_gvar_to_gvar() {
        assert_parses_rule!(
            Alias,
            b"alias $foo $bar",
            r#"
s(:alias,
  s(:gvar, "$foo"),
  s(:gvar, "$bar"))
            "#
        )
    }
}
