use crate::{
    builder::Builder,
    parser::{macros::one_of, ParseResult, Parser},
    token::TokenKind,
    Node,
};

use super::macros::{all_of, at_least_once, maybe::maybe};

impl Parser {
    pub(crate) fn parse_strings(&mut self) -> ParseResult<Box<Node>> {
        one_of!("strings", self.parse_char(), self.parse_string_seq(),)
    }

    fn parse_char(&mut self) -> ParseResult<Box<Node>> {
        let char_t = self.try_token(TokenKind::tCHAR)?;
        Ok(Builder::character(char_t))
    }

    fn parse_string_seq(&mut self) -> ParseResult<Box<Node>> {
        let parts = at_least_once!("string", self.parse_string1())?;

        Ok(Builder::string_compose(None, parts, None))
    }

    fn parse_string1(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, parts, end_t) = all_of!(
            "string1",
            one_of!(
                "string1 begin",
                self.try_token(TokenKind::tSTRING_BEG),
                self.try_token(TokenKind::tDSTRING_BEG),
                self.try_token(TokenKind::tHEREDOC_BEG),
            ),
            self.parse_string_contents(),
            self.expect_token(TokenKind::tSTRING_END),
        )?;

        // TODO: dedent_heredoc
        Ok(Builder::string_compose(Some(begin_t), parts, Some(end_t)))
    }

    // This rule can be `none`
    pub(crate) fn parse_string_contents(&mut self) -> ParseResult<Vec<Node>> {
        let contents = maybe!(at_least_once!(
            "string_contents",
            self.parse_string_content()
        ))?;

        Ok(contents.unwrap_or_else(|| vec![]))
    }

    pub(crate) fn parse_string_content(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "string_content",
            checkpoint = self.new_checkpoint(),
            {
                let string_content_t = self.try_token(TokenKind::tSTRING_CONTENT)?;
                Ok(Builder::string_internal(string_content_t, self.buffer()))
            },
            {
                let (_string_dvar_t, string_dvar) = all_of!(
                    "tSTRING_DVAR string_dvar",
                    self.try_token(TokenKind::tSTRING_DVAR),
                    self.parse_string_dvar(),
                )?;

                Ok(string_dvar)
            },
            {
                let (begin_t, compstmt, end_t) = all_of!(
                    "tSTRING_DBEG compstmt tSTRING_DEND",
                    self.try_token(TokenKind::tSTRING_DBEG),
                    self.try_compstmt(),
                    self.expect_token(TokenKind::tSTRING_DEND),
                )?;

                let stmts = if let Some(compstmt) = compstmt {
                    vec![*compstmt]
                } else {
                    vec![]
                };

                Ok(Builder::begin(begin_t, stmts, end_t))
            },
        )
    }

    fn parse_string_dvar(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "string_dvar",
            self.parse_gvar(),
            self.parse_ivar(),
            self.parse_cvar(),
            self.parse_back_ref(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_char() {
        assert_parses!(parse_strings, b"?\\u0001", "s(:str, \"\\u{1}\")")
    }

    #[test]
    fn test_string1_plain() {
        assert_parses!(parse_strings, b"'foo'", "s(:str, \"foo\")");
    }

    #[test]
    fn test_string1_interp() {
        assert_parses!(
            parse_strings,
            b"\"foo #{42} #@bar\"",
            r#"
s(:dstr,
  s(:str, "foo "),
  s(:begin,
    s(:int, "42")),
  s(:str, " "),
  s(:ivar, "@bar"))
            "#
        );
    }
}
