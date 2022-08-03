use crate::{
    builder::Builder,
    parser::{ParseError, ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_strings(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("strings")
            .or_else(|| self.parse_char())
            .or_else(|| self.parse_string_seq())
            .stop()
    }

    fn parse_char(&mut self) -> ParseResult<Box<Node>> {
        let char_t = self.parse_token(TokenKind::tCHAR)?;
        Ok(Builder::character(char_t))
    }

    fn parse_string_seq(&mut self) -> ParseResult<Box<Node>> {
        let mut parts = vec![];

        let string = self.parse_string1()?;
        parts.push(*string);

        loop {
            match self.parse_string1() {
                Ok(string) => {
                    parts.push(*string);
                }
                Err(error) => {
                    match error.strip_lookaheads() {
                        None => {
                            // no match
                            break;
                        }
                        Some(error) => {
                            return Err(ParseError::seq_error::<Box<Node>, _>(
                                "string1", parts, error,
                            ));
                        }
                    }
                }
            }
        }

        Ok(Builder::string_compose(None, parts, None))
    }

    fn parse_string1(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, parts, end_t) = self
            .all_of("string1")
            .and(|| {
                self.one_of("string begin")
                    .or_else(|| self.parse_token(TokenKind::tDSTRING_BEG))
                    .or_else(|| self.parse_token(TokenKind::tSTRING_BEG))
                    .or_else(|| self.parse_token(TokenKind::tHEREDOC_BEG))
                    .stop()
            })
            .and(|| self.parse_string_contents())
            .and(|| self.expect_token(TokenKind::tSTRING_END))
            .stop()?;

        // TODO: dedent_heredoc
        Ok(Builder::string_compose(Some(begin_t), parts, Some(end_t)))
    }

    // This rule can be `none`
    pub(crate) fn parse_string_contents(&mut self) -> ParseResult<Vec<Node>> {
        let mut strings = vec![];
        loop {
            if self.current_token().is(TokenKind::tSTRING_END) {
                break;
            }

            match self.parse_string_content() {
                Ok(string) => {
                    strings.push(*string);
                }
                Err(error) => {
                    match error.strip_lookaheads() {
                        None => {
                            // no match
                            break;
                        }
                        Some(error) => {
                            return Err(ParseError::seq_error::<Vec<Node>, _>(
                                "string content",
                                strings,
                                error,
                            ));
                        }
                    }
                }
            }
        }
        Ok(strings)
    }

    pub(crate) fn parse_string_content(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("string content")
            .or_else(|| {
                let string_content_t = self.parse_token(TokenKind::tSTRING_CONTENT)?;
                Ok(Builder::string_internal(string_content_t, self.buffer()))
            })
            .or_else(|| {
                let (_string_dvar_t, string_dvar) = self
                    .all_of("string dvar")
                    .and(|| self.parse_token(TokenKind::tSTRING_DVAR))
                    .and(|| self.parse_string_dvar())
                    .stop()?;

                Ok(string_dvar)
            })
            .or_else(|| {
                let (string_dbeg_t, compstmt, string_dend_t) = self
                    .all_of("#{ stmt }")
                    .and(|| self.parse_token(TokenKind::tSTRING_DBEG))
                    .and(|| self.try_compstmt())
                    .and(|| self.expect_token(TokenKind::tSTRING_DEND))
                    .stop()?;

                Ok(Builder::begin(string_dbeg_t, compstmt, string_dend_t))
            })
            .stop()
    }

    fn parse_string_dvar(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("string_dvar")
            .or_else(|| self.parse_gvar())
            .or_else(|| self.parse_ivar())
            .or_else(|| self.parse_cvar())
            .or_else(|| self.parse_back_ref())
            .stop()
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
