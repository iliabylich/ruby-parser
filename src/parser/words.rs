use crate::{
    builder::Builder,
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_words(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, elements, end_t) = self
            .all_of("words")
            .and(|| self.parse_token(TokenKind::tWORDS_BEG))
            .and(|| self.parse_word_list())
            .and(|| self.expect_token(TokenKind::tSTRING_END))
            .stop()?;

        Ok(Builder::words_compose(begin_t, elements, end_t))
    }

    // This rule can be `none
    fn parse_word_list(&mut self) -> ParseResult<Vec<Node>> {
        let mut result = vec![];
        while let Some(word) = self.try_word()? {
            result.push(*word)
        }
        Ok(result)
    }

    pub(crate) fn try_word(&mut self) -> ParseResult<Option<Box<Node>>> {
        let contents = self.parse_string_contents()?;
        if contents.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::word(contents)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_words() {
        assert_parses!(parse_words, b"%W[foo bar]", "TODO")
    }
}
