use crate::{
    builder::{Builder, Constructor},
    lexer::strings::{literal::StringLiteral, types::Regexp},
    parser::Parser,
    token::{Token, TokenValue},
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_symbols(&mut self) -> Option<Box<Node<'a>>> {
        let begin_t = self.try_token(TokenValue::tSYMBOLS_BEG)?;
        let word_list = self.parse_symbol_list();
        let end_t = self.expect_token(TokenValue::tSTRING_END);
        Some(Builder::<C>::symbols_compose(begin_t, word_list, end_t))
    }

    fn parse_symbol_list(&mut self) -> Vec<Node<'a>> {
        let mut result = vec![];
        while let Some(word) = self.try_word() {
            result.push(*word);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{loc::loc, string_content::StringContent, Node, RustParser};

    #[test]
    fn test_words() {
        let mut parser = RustParser::new(b"%I[foo bar]");
        assert_eq!(parser.parse(), None);
        todo!("implement me");
    }
}
