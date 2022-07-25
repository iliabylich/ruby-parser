use crate::{
    builder::{Builder, Constructor},
    lexer::strings::{literal::StringLiteral, types::Regexp},
    parser::{ParseError, Parser},
    token::{Token, TokenKind},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_qwords(&mut self) -> Result<Box<Node>, ParseError> {
        let (begin_t, word_list, end_t) = self
            .all_of("qwords")
            .and(|| self.try_token(TokenKind::tQWORDS_BEG))
            .and(|| Ok(self.parse_qword_list()))
            .and(|| self.expect_token(TokenKind::tSTRING_END))
            .unwrap()?;

        Ok(Builder::<C>::words_compose(begin_t, word_list, end_t))
    }

    // This rule can be `None`
    fn parse_qword_list(&mut self) -> Vec<Node> {
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
    use crate::{loc::loc, parser::ParseError, string_content::StringContent, Node, RustParser};

    #[test]
    fn test_qwords() {
        let mut parser = RustParser::new(b"%w[foo bar]");
        assert_eq!(parser.try_qwords(), Err(ParseError::empty()));
        todo!("implement me");
    }
}
