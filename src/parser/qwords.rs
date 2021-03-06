use crate::{
    builder::{Builder, Constructor},
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_qwords(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, word_list, end_t) = self
            .all_of("qwords")
            .and(|| self.try_token(TokenKind::tQWORDS_BEG))
            .and(|| self.try_qword_list())
            .and(|| self.expect_token(TokenKind::tSTRING_END))
            .stop()?;

        Ok(Builder::<C>::words_compose(begin_t, word_list, end_t))
    }

    // This rule can be `None`
    fn try_qword_list(&mut self) -> ParseResult<Vec<Node>> {
        let mut result = vec![];
        loop {
            if let Ok(string_t) = self.try_token(TokenKind::tSTRING_CONTENT) {
                let node = Builder::<C>::string_internal(string_t, self.buffer());
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
        assert_parses!(try_qwords, b"%w[foo bar]", "TODO")
    }
}
