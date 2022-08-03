use crate::{
    builder::Builder,
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn try_words(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, elements, end_t) = self
            .all_of("words")
            .and(|| self.try_token(TokenKind::tWORDS_BEG))
            .and(|| self.try_word_list())
            .and(|| self.expect_token(TokenKind::tSTRING_END))
            .stop()?;

        Ok(Builder::words_compose(begin_t, elements, end_t))
    }

    // This rule can be `none
    fn try_word_list(&mut self) -> ParseResult<Vec<Node>> {
        let mut result = vec![];
        while let Some(word) = self.try_word()? {
            result.push(*word)
        }
        Ok(result)
    }

    pub(crate) fn try_word(&mut self) -> ParseResult<Option<Box<Node>>> {
        let contents = self.try_string_contents()?;
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
        assert_parses!(try_words, b"%W[foo bar]", "TODO")
    }
}
