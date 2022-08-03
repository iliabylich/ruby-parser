use crate::{
    builder::Builder,
    parser::{ParseError, ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn try_strings(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("strings")
            .or_else(|| self.try_char())
            .or_else(|| self.try_string_seq())
            .stop()
    }

    fn try_char(&mut self) -> ParseResult<Box<Node>> {
        let char_t = self.try_token(TokenKind::tCHAR)?;
        Ok(Builder::character(char_t))
    }

    fn try_string_seq(&mut self) -> ParseResult<Box<Node>> {
        let mut parts = vec![];

        let string = self.try_string1()?;
        parts.push(*string);

        loop {
            match self.try_string1() {
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

    fn try_string1(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, parts, end_t) = self
            .all_of("string1")
            .and(|| {
                self.one_of("string begin")
                    .or_else(|| self.try_token(TokenKind::tDSTRING_BEG))
                    .or_else(|| self.try_token(TokenKind::tSTRING_BEG))
                    .or_else(|| self.try_token(TokenKind::tHEREDOC_BEG))
                    .stop()
            })
            .and(|| self.try_string_contents())
            .and(|| self.expect_token(TokenKind::tSTRING_END))
            .stop()?;

        // TODO: dedent_heredoc
        Ok(Builder::string_compose(Some(begin_t), parts, Some(end_t)))
    }

    // This rule can be `none`
    pub(crate) fn try_string_contents(&mut self) -> ParseResult<Vec<Node>> {
        let mut strings = vec![];
        loop {
            if self.current_token().is(TokenKind::tSTRING_END) {
                break;
            }

            match self.try_string_content() {
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

    pub(crate) fn try_string_content(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("string content")
            .or_else(|| {
                let string_content_t = self.try_token(TokenKind::tSTRING_CONTENT)?;
                Ok(Builder::string_internal(string_content_t, self.buffer()))
            })
            .or_else(|| {
                let (_string_dvar_t, string_dvar) = self
                    .all_of("string dvar")
                    .and(|| self.try_token(TokenKind::tSTRING_DVAR))
                    .and(|| self.try_string_dvar())
                    .stop()?;

                Ok(string_dvar)
            })
            .or_else(|| {
                let (string_dbeg_t, compstmt, string_dend_t) = self
                    .all_of("#{ stmt }")
                    .and(|| self.try_token(TokenKind::tSTRING_DBEG))
                    .and(|| self.try_compstmt())
                    .and(|| self.expect_token(TokenKind::tSTRING_DEND))
                    .stop()?;

                Ok(Builder::begin(string_dbeg_t, compstmt, string_dend_t))
            })
            .stop()
    }

    fn try_string_dvar(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("string_dvar")
            .or_else(|| self.try_gvar())
            .or_else(|| self.try_ivar())
            .or_else(|| self.try_cvar())
            .or_else(|| self.try_back_ref())
            .stop()
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_char() {
        assert_parses!(try_strings, b"?\\u0001", "s(:str, \"\\u{1}\")")
    }

    #[test]
    fn test_string1_plain() {
        assert_parses!(try_strings, b"'foo'", "s(:str, \"foo\")");
    }

    #[test]
    fn test_string1_interp() {
        assert_parses!(
            try_strings,
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
