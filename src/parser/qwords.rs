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
    pub(crate) fn parse_qwords(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, word_list, end_t) = all_of!(
            "qwords",
            self.try_token(TokenKind::tQWORDS_BEG),
            parse_qword_list(self),
            self.expect_token(TokenKind::tSTRING_END),
        )?;

        Ok(Builder::words_compose(begin_t, word_list, end_t))
    }
}

// This rule can be `None`
fn parse_qword_list(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    let qword_list = maybe!(separated_by!(
        "qword list",
        checkpoint = parser.new_checkpoint(),
        item = parser
            .try_token(TokenKind::tSTRING_CONTENT)
            .map(|token| Builder::string_internal(token, parser.buffer())),
        sep = parser.try_token(TokenKind::tSP)
    ))?;

    match qword_list {
        Some((qword_list, _spaces)) => Ok(qword_list),
        None => Ok(vec![]),
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_qwords_empty() {
        assert_parses!(Parser::parse_qwords, b"%w[]", "s(:array)")
    }

    #[test]
    fn test_qwords() {
        assert_parses!(
            Parser::parse_qwords,
            b"%w[foo bar]",
            r#"
s(:array,
  s(:str, "foo"),
  s(:str, "bar"))
            "#
        )
    }
}
