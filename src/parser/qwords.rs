use crate::{
    builder::Builder,
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_qwords(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, word_list, end_t) = self
            .all_of("qwords")
            .and(|| self.try_token(TokenKind::tQWORDS_BEG))
            .and(|| self.parse_qword_list())
            .and(|| self.expect_token(TokenKind::tSTRING_END))
            .stop()?;

        Ok(Builder::words_compose(begin_t, word_list, end_t))
    }

    // This rule can be `None`
    fn parse_qword_list(&mut self) -> ParseResult<Vec<Node>> {
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
    fn test_qwords() {
        assert_parses!(parse_qwords, b"%w[foo bar]", "TODO")
    }
}
