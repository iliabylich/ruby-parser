use crate::{
    builder::Builder,
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_qsymbols(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, word_list, end_t) = self
            .all_of("qsymbols")
            .and(|| self.try_token(TokenKind::tQSYMBOLS_BEG))
            .and(|| self.parse_qsym_list())
            .and(|| self.expect_token(TokenKind::tSTRING_END))
            .stop()?;

        Ok(Builder::symbols_compose(begin_t, word_list, end_t))
    }

    // This rule can be `None`
    fn parse_qsym_list(&mut self) -> ParseResult<Vec<Node>> {
        let mut result = vec![];
        loop {
            if let Ok(string_t) = self.try_token(TokenKind::tSTRING_CONTENT) {
                let node = Builder::string_internal(string_t, self.buffer());
                result.push(*node);
            } else {
                break;
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_qsymbols() {
        assert_parses!(parse_qsymbols, b"%i[foo bar]", "TODO")
    }
}
