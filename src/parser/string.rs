use crate::{
    builder::Builder,
    parser::{ParseError, ParseResult, Parser},
    token::TokenKind,
    Node, Token,
};

impl Parser {
    pub(crate) fn parse_strings(&mut self) -> ParseResult<Box<Node>> {
        match self.parse_char() {
            Ok(node) => Ok(node),
            Err(error1) => match self.parse_string_seq() {
                Ok(node) => Ok(node),
                Err(error2) => Err(ParseError::OneOfError {
                    name: "strings",
                    variants: vec![error1, error2],
                }),
            },
        }
    }

    fn parse_char(&mut self) -> ParseResult<Box<Node>> {
        let char_t = self.try_token(TokenKind::tCHAR)?;
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
                            return Err(ParseError::seq_error("string_seq", parts, error));
                        }
                    }
                }
            }
        }

        Ok(Builder::string_compose(None, parts, None))
    }

    fn parse_string1(&mut self) -> ParseResult<Box<Node>> {
        let begin_t = match self.current_token() {
            Token {
                kind: TokenKind::tDSTRING_BEG | TokenKind::tSTRING_BEG | TokenKind::tHEREDOC_BEG,
                ..
            } => self.current_token(),
            Token { kind: got, loc, .. } => {
                return Err(ParseError::OneOfError {
                    name: "string1",
                    variants: vec![
                        ParseError::TokenError {
                            lookahead: true,
                            expected: TokenKind::tDSTRING_BEG,
                            got,
                            loc,
                        },
                        ParseError::TokenError {
                            lookahead: true,
                            expected: TokenKind::tSTRING_BEG,
                            got,
                            loc,
                        },
                        ParseError::TokenError {
                            lookahead: true,
                            expected: TokenKind::tHEREDOC_BEG,
                            got,
                            loc,
                        },
                    ],
                });
            }
        };
        self.skip_token();

        match self.parse_string_contents() {
            Ok(parts) => {
                match self.expect_token(TokenKind::tSTRING_END) {
                    Ok(end_t) => {
                        // TODO: dedent_heredoc
                        Ok(Builder::string_compose(Some(begin_t), parts, Some(end_t)))
                    }
                    Err(error) => Err(ParseError::seq_error("string1", (begin_t, parts), error)),
                }
            }
            Err(error) => Err(ParseError::seq_error("string1", begin_t, error)),
        }
    }

    // This rule can be `none`
    pub(crate) fn parse_string_contents(&mut self) -> ParseResult<Vec<Node>> {
        let mut strings = vec![];
        loop {
            if self.current_token().is(TokenKind::tSTRING_END) {
                break;
            }

            match self
                .parse_string_content()
                .map_err(|err| err.strip_lookaheads())
            {
                Ok(string) => {
                    strings.push(*string);
                }
                Err(None) => {
                    // no match
                    break;
                }
                Err(Some(error)) => {
                    return Err(ParseError::seq_error("string content", strings, error));
                }
            }
        }
        Ok(strings)
    }

    pub(crate) fn parse_string_content(&mut self) -> ParseResult<Box<Node>> {
        match self.current_token() {
            string_content_t @ Token {
                kind: TokenKind::tSTRING_CONTENT,
                ..
            } => {
                self.skip_token();
                Ok(Builder::string_internal(string_content_t, self.buffer()))
            }

            string_dvar_t @ Token {
                kind: TokenKind::tSTRING_DVAR,
                ..
            } => {
                self.skip_token();
                match self.parse_string_dvar() {
                    Ok(string_dvar) => Ok(string_dvar),
                    Err(error) => Err(ParseError::seq_error("string dvar", string_dvar_t, error)),
                }
            }

            string_dbeg_t @ Token {
                kind: TokenKind::tSTRING_DBEG,
                ..
            } => {
                self.skip_token();
                match self.try_compstmt() {
                    Ok(compstmt) => match self.expect_token(TokenKind::tSTRING_DEND) {
                        Ok(string_dend_t) => {
                            let stmts = if let Some(compstmt) = compstmt {
                                vec![*compstmt]
                            } else {
                                vec![]
                            };

                            Ok(Builder::begin(string_dbeg_t, stmts, string_dend_t))
                        }
                        Err(error) => Err(ParseError::seq_error(
                            "#{ interpolated stmt }",
                            (string_dbeg_t, compstmt),
                            error,
                        )),
                    },
                    Err(error) => Err(ParseError::seq_error(
                        "#{ interpolated stmt }",
                        string_dbeg_t,
                        error,
                    )),
                }
            }

            Token { kind: got, loc, .. } => Err(ParseError::OneOfError {
                name: "string content",
                variants: vec![
                    ParseError::TokenError {
                        lookahead: true,
                        expected: TokenKind::tSTRING_CONTENT,
                        got,
                        loc,
                    },
                    ParseError::TokenError {
                        lookahead: true,
                        expected: TokenKind::tSTRING_DVAR,
                        got,
                        loc,
                    },
                    ParseError::TokenError {
                        lookahead: true,
                        expected: TokenKind::tSTRING_DBEG,
                        got,
                        loc,
                    },
                ],
            }),
        }
    }

    fn parse_string_dvar(&mut self) -> ParseResult<Box<Node>> {
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
