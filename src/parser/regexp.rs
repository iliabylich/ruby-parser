use crate::{
    builder::Builder,
    lexer::strings::{literal::StringLiteral, types::Regexp},
    loc::loc,
    parser::{
        macros::{all_of, one_of},
        ParseResult, Parser,
    },
    token::{token, Token, TokenKind},
    transactions::ParseError,
    Node,
};

impl Parser {
    pub(crate) fn parse_regexp(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, contents, end_t) = all_of!(
            "regexp",
            {
                one_of!(
                    "regexp",
                    checkpoint = self.new_checkpoint(),
                    self.try_token(TokenKind::tREGEXP_BEG),
                    {
                        let token = read_div_as_heredoc_beg(self)?;

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
                    },
                )
            },
            parse_regexp_contents(self),
            self.expect_token(TokenKind::tSTRING_END),
        )?;

        let options = Builder::regexp_options(&end_t, self.buffer());
        Ok(Builder::regexp_compose(begin_t, contents, end_t, options))
    }
}

// This rule can be `none`
fn parse_regexp_contents(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    parser.parse_string_contents()
}

fn read_div_as_heredoc_beg(parser: &mut Parser) -> ParseResult<Token> {
    let loc = parser.current_token().loc;
    if parser.current_token().is(TokenKind::tDIVIDE) {
        let token = token!(TokenKind::tREGEXP_BEG, loc!(loc.start, loc.end));
        parser.lexer.tokens_mut()[parser.lexer.token_idx()] = token;
        parser.skip_token();
        Ok(token)
    } else {
        Err(ParseError::TokenError {
            lookahead: true,
            expected: TokenKind::tREGEXP_BEG,
            got: parser.current_token().kind,
            loc: parser.current_token().loc,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_regexp() {
        assert_parses!(
            parse_regexp,
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
            parse_regexp,
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
            parse_regexp,
            b"%r{foo}",
            r#"
s(:regexp,
  s(:str, "foo"),
  s(:regopt))
        "#
        )
    }
}
