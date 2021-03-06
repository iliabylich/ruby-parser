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
    pub(crate) fn try_symbols(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, elements, end_t) = self
            .all_of("symbols")
            .and(|| self.try_token(TokenKind::tSYMBOLS_BEG))
            .and(|| try_symbol_list(self))
            .and(|| self.expect_token(TokenKind::tSTRING_END))
            .stop()?;

        Ok(Builder::<C>::symbols_compose(begin_t, elements, end_t))
    }
}

// This rule can be `none`
fn try_symbol_list<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Vec<Node>> {
    let mut result = vec![];
    while let Some(word) = parser.try_word()? {
        result.push(*word);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_symbols() {
        assert_parses!(try_symbols, b"%I[foo bar]", "TODO")
    }
}
