use crate::{
    builder::Builder,
    lexer::strings::{
        literal::StringLiteral,
        types::{Interpolation, StringInterp},
    },
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
    pub(crate) fn parse_xstring(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, parts, end_t) = all_of!(
            "xstring",
            {
                one_of!(
                    "executable string begin",
                    checkpoint = self.new_checkpoint(),
                    read_backtick_identifier_as_xstring_beg(self),
                    self.try_token(TokenKind::tXHEREDOC_BEG),
                )
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
            },
            parse_xstring_contents(self),
            self.expect_token(TokenKind::tSTRING_END),
        )?;

        Ok(Builder::xstring_compose(begin_t, parts, end_t))
    }
}

// This rule can be `none`
fn parse_xstring_contents(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    parser.parse_string_contents()
}

fn read_backtick_identifier_as_xstring_beg(parser: &mut Parser) -> ParseResult<Token> {
    let loc = parser.current_token().loc;
    if parser.current_token().is(TokenKind::tIDENTIFIER) {
        if parser.buffer().slice(loc.start, loc.end) == Some(b"`") {
            let token = token!(TokenKind::tXSTRING_BEG, loc!(loc.start, loc.end));
            parser.lexer.tokens_mut()[parser.lexer.token_idx()] = token;
            parser.skip_token();
            return Ok(token);
        }
    }
    Err(ParseError::TokenError {
        lookahead: true,
        expected: TokenKind::tXSTRING_BEG,
        got: parser.current_token().kind,
        loc: parser.current_token().loc,
    })
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_xstring_plain() {
        assert_parses!(
            Parser::parse_xstring,
            b"`foo`",
            r#"
s(:xstr,
  s(:str, "foo"))
            "#
        );
    }
}
