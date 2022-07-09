use crate::{
    builder::{Builder, Constructor},
    lexer::strings::{literal::StringLiteral, types::Regexp},
    parser::Parser,
    token::{Token, TokenKind},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_qsymbols(&mut self) -> Option<Box<Node>> {
        let begin_t = self.try_token(TokenKind::tSYMBOLS_BEG)?;
        let word_list = self.parse_qsym_list();
        let end_t = self.expect_token(TokenKind::tSTRING_END);
        Some(Builder::<C>::symbols_compose(begin_t, word_list, end_t))
    }

    // This rule can be `None`
    fn parse_qsym_list(&mut self) -> Vec<Node> {
        let mut result = vec![];
        loop {
            if self.current_token().is(TokenKind::tSTRING_CONTENT) {
                let string_t = self.take_token();
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
    use crate::{loc::loc, string_content::StringContent, Node, RustParser};

    #[test]
    fn test_words() {
        let mut parser = RustParser::new(b"%i[foo bar]");
        assert_eq!(parser.parse(), None);
        todo!("implement me");
    }
}
