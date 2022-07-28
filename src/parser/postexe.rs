use crate::{
    builder::{Builder, Constructor},
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_postexe(&mut self) -> ParseResult<Box<Node>> {
        let (postexe_t, lcurly_t, compstmt, rcurly_t) = self
            .all_of("postexe")
            .and(|| self.try_token(TokenKind::klEND))
            .and(|| self.expect_token(TokenKind::tLCURLY))
            .and(|| self.try_compstmt())
            .and(|| self.expect_token(TokenKind::tRCURLY))
            .stop()?;

        Ok(Builder::<C>::postexe(
            postexe_t, lcurly_t, compstmt, rcurly_t,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_postexe() {
        assert_parses!(
            try_postexe,
            b"END { 42 }",
            r#"
s(:postexe,
  s(:int, "42"))
        "#
        )
    }

    #[test]
    fn test_postexe_empty() {
        assert_parses!(try_postexe, b"END {}", "s(:postexe)")
    }
}
