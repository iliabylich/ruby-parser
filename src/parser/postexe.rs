use crate::{
    builder::Builder,
    parser::{macros::all_of, ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_postexe(&mut self) -> ParseResult<Box<Node>> {
        let (postexe_t, lcurly_t, compstmt, rcurly_t) = all_of!(
            "postexe",
            self.try_token(TokenKind::klEND),
            self.expect_token(TokenKind::tLCURLY),
            self.try_compstmt(),
            self.expect_token(TokenKind::tRCURLY),
        )?;

        Ok(Builder::postexe(postexe_t, lcurly_t, compstmt, rcurly_t))
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_postexe() {
        assert_parses!(
            Parser::parse_postexe,
            b"END { 42 }",
            r#"
s(:postexe,
  s(:int, "42"))
        "#
        )
    }

    #[test]
    fn test_postexe_empty() {
        assert_parses!(Parser::parse_postexe, b"END {}", "s(:postexe)")
    }
}
