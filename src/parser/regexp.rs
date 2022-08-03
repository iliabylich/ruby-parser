use crate::{
    builder::Builder,
    lexer::strings::{literal::StringLiteral, types::Regexp},
    loc::loc,
    parser::{ParseResult, Parser},
    token::{token, Token, TokenKind},
    transactions::ParseError,
    Node,
};

impl Parser {
    pub(crate) fn try_regexp(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, contents, end_t) = self
            .all_of("regexp")
            .and(|| {
                self.one_of("regexp")
                    .or_else(|| self.try_token(TokenKind::tREGEXP_BEG))
                    .or_else(|| {
                        let token = self.read_div_as_heredoc_beg()?;

                        // now we need to manually push a xstring literal
                        // Lexer is not capable of doing it
                        self.lexer
                            .string_literals()
                            .push(StringLiteral::Regexp(Regexp::new(
                                b'/',
                                b'/',
                                self.lexer.curly_nest(),
                            )));

                        Ok(token)
                    })
                    .stop()
            })
            .and(|| self.try_regexp_contents())
            .and(|| self.expect_token(TokenKind::tSTRING_END))
            .stop()?;

        let options = Builder::regexp_options(&end_t, self.buffer());
        Ok(Builder::regexp_compose(begin_t, contents, end_t, options))
    }

    // This rule can be `none`
    fn try_regexp_contents(&mut self) -> ParseResult<Vec<Node>> {
        self.try_string_contents()
    }

    fn read_div_as_heredoc_beg(&mut self) -> ParseResult<Token> {
        let loc = self.current_token().loc;
        if self.current_token().is(TokenKind::tDIVIDE) {
            let token = token!(TokenKind::tREGEXP_BEG, loc!(loc.start, loc.end));
            self.lexer.tokens_mut()[self.lexer.token_idx()] = token;
            self.skip_token();
            Ok(token)
        } else {
            Err(ParseError::TokenError {
                lookahead: true,
                expected: TokenKind::tREGEXP_BEG,
                got: self.current_token().kind,
                loc: self.current_token().loc,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_regexp() {
        assert_parses!(
            try_regexp,
            b"/foo/",
            r#"
s(:regexp,
  s(:str, "foo"),
  s(:regopt))
            "#
        )
    }

    #[test]
    fn test_regexp_with_options() {
        assert_parses!(
            try_regexp,
            b"/foo/mix",
            r#"
s(:regexp,
  s(:str, "foo"),
  s(:regopt, "i", "m", "x"))
        "#
        )
    }

    #[test]
    fn test_regexp_percent() {
        assert_parses!(
            try_regexp,
            b"%r{foo}",
            r#"
s(:regexp,
  s(:str, "foo"),
  s(:regopt))
        "#
        )
    }
}
