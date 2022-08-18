use crate::{
    builder::{Builder, KeywordCmd},
    parser::{macros::all_of, ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_defined(&mut self) -> ParseResult<Box<Node>> {
        let (defined_t, _nl, lparen_t, expr, rparen_t) = all_of!(
            "defined? value",
            self.try_token(TokenKind::kDEFINED),
            self.try_opt_nl(),
            self.expect_token(TokenKind::tLPAREN),
            self.parse_expr(),
            self.expect_token(TokenKind::tRPAREN),
        )?;

        Ok(Builder::keyword_cmd(
            KeywordCmd::Defined,
            defined_t,
            Some(lparen_t),
            vec![*expr],
            Some(rparen_t),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_defined() {
        assert_parses!(
            parse_defined,
            b"defined?(42)",
            r#"
s(:defined?,
  s(:int, "42"))
            "#
        )
    }
}
