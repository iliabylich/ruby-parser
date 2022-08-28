use crate::{
    builder::Builder,
    parser::{
        macros::{all_of, maybe, separated_by},
        ParseResult, Parser,
    },
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_qsymbols(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, word_list, end_t) = all_of!(
            "qsymbols",
            self.try_token(TokenKind::tQSYMBOLS_BEG),
            parse_qsym_list(self),
            self.expect_token(TokenKind::tSTRING_END),
        )?;

        Ok(Builder::symbols_compose(begin_t, word_list, end_t))
    }
}

// This rule can be `None`
fn parse_qsym_list(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    let qsym_list = maybe!(separated_by!(
        "qsym list",
        checkpoint = parser.new_checkpoint(),
        item = parser
            .try_token(TokenKind::tSTRING_CONTENT)
            .map(|token| Builder::symbol_internal(token, parser.buffer())),
        sep = parser.try_token(TokenKind::tSP)
    ))?;

    match qsym_list {
        Some((qsym_list, _spaces)) => Ok(qsym_list),
        None => Ok(vec![]),
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_qsymbols_empty() {
        assert_parses!(Parser::parse_qsymbols, b"%i[]", "s(:array)")
    }

    #[test]
    fn test_qsymbols() {
        assert_parses!(
            Parser::parse_qsymbols,
            b"%i[foo bar]",
            r#"
s(:array,
  s(:sym, "foo"),
  s(:sym, "bar"))
            "#
        )
    }
}
