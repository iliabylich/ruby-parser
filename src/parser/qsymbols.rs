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
    pub(crate) fn try_qsymbols(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, word_list, end_t) = self
            .all_of("qsymbols")
            .and(|| self.try_token(TokenKind::tQSYMBOLS_BEG))
            .and(|| Ok(self.try_qsym_list()))
            .and(|| self.expect_token(TokenKind::tSTRING_END))
            .unwrap()?;

        Ok(Builder::<C>::symbols_compose(begin_t, word_list, end_t))
    }

    // This rule can be `None`
    fn try_qsym_list(&mut self) -> Vec<Node> {
        let mut result = vec![];
        loop {
            if self.current_token().is(TokenKind::tSTRING_CONTENT) {
                let string_t = self.current_token();
                self.skip_token();
                let node = Builder::<C>::string_internal(string_t, self.buffer());
                result.push(*node);
            } else {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::ParseError, RustParser};

    #[test]
    fn test_qsymbols() {
        let mut parser = RustParser::new(b"%i[foo bar]");
        assert_eq!(parser.try_qsymbols(), Err(ParseError::empty()));
        todo!("implement me");
    }
}
