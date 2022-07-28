use crate::{
    builder::{Builder, Constructor},
    lexer::strings::{
        literal::StringLiteral,
        types::{Interpolation, StringInterp},
    },
    loc::loc,
    parser::{ParseResult, Parser},
    token::{token, Token, TokenKind},
    transactions::ParseError,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_xstring(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, parts, end_t) = self
            .all_of("xstring")
            .and(|| {
                self.one_of("executable string begin")
                    .or_else(|| self.read_backtick_identifier_as_xstring_beg())
                    .or_else(|| self.try_token(TokenKind::tXHEREDOC_BEG))
                    .stop()
                    .and_then(|tok| {
                        // now we need to manually push a xstring literal
                        // Lexer is not capable of doing it
                        self.lexer
                            .string_literals()
                            .push(StringLiteral::StringInterp(StringInterp::new(
                                Interpolation::new(self.lexer.curly_nest()),
                                b'`',
                                b'`',
                            )));
                        Ok(tok)
                    })
            })
            .and(|| self.try_xstring_contents())
            .and(|| self.expect_token(TokenKind::tSTRING_END))
            .stop()?;

        Ok(Builder::<C>::xstring_compose(begin_t, parts, end_t))
    }

    // This rule can be `none`
    fn try_xstring_contents(&mut self) -> ParseResult<Vec<Node>> {
        self.try_string_contents()
    }

    fn read_backtick_identifier_as_xstring_beg(&mut self) -> ParseResult<Token> {
        let loc = self.current_token().loc;
        if self.current_token().is(TokenKind::tIDENTIFIER) {
            if self.buffer().slice(loc.start, loc.end) == Some(b"`") {
                let token = token!(TokenKind::tXSTRING_BEG, loc!(loc.start, loc.end));
                self.lexer.tokens_mut()[self.lexer.token_idx()] = token;
                self.skip_token();
                return Ok(token);
            }
        }
        Err(ParseError::TokenError {
            lookahead: true,
            expected: TokenKind::tXSTRING_BEG,
            got: self.current_token().kind,
            loc: self.current_token().loc,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_xstring_plain() {
        assert_parses!(
            try_xstring,
            b"`foo`",
            r#"
s(:xstr,
  s(:str, "foo"))
            "#
        );
    }
}
