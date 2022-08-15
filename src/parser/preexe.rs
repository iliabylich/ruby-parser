use crate::{
    builder::Builder,
    parser::{macros::all_of, ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_preexe(&mut self) -> ParseResult<Box<Node>> {
        let (preexe_t, lcurly_t, body, rcurly_t) = all_of!(
            "preexe",
            self.try_token(TokenKind::klBEGIN),
            self.expect_token(TokenKind::tLCURLY),
            self.try_top_compstmt(),
            self.expect_token(TokenKind::tRCURLY),
        )?;

        Ok(Builder::preexe(preexe_t, lcurly_t, body, rcurly_t))
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_parses, assert_parses_with_error};

    #[test]
    fn test_preexe() {
        assert_parses!(
            parse_preexe,
            b"BEGIN { 42 }",
            r#"
s(:preexe,
  s(:int, "42"))
        "#
        );
    }

    #[test]
    fn test_preexe_empty() {
        assert_parses!(parse_preexe, b"BEGIN {}", "s(:preexe)");
    }

    #[test]
    fn test_nothing() {
        assert_parses_with_error!(
            parse_postexe,
            b"",
            "
SEQUENCE (0) postexe (got [])
    TOKEN (0) expected klEND, got tEOF (at 0)
    "
        );
    }
}
