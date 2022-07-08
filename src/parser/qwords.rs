use crate::{
    builder::{Builder, Constructor},
    lexer::strings::{literal::StringLiteral, types::Regexp},
    parser::Parser,
    token::{Token, TokenKind},
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_qwords(&mut self) -> Option<Box<Node<'a>>> {
        let begin_t = self.try_token(TokenKind::tQWORDS_BEG)?;
        let word_list = self.parse_qword_list();
        let end_t = self.expect_token(TokenKind::tSTRING_END);
        Some(Builder::<C>::words_compose(begin_t, word_list, end_t))
    }

    // This rule can be `None`
    fn parse_qword_list(&mut self) -> Vec<Node<'a>> {
        let mut result = vec![];
        loop {
            if matches!(self.current_token().kind(), TokenKind::tSTRING_CONTENT(_)) {
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
        let mut parser = RustParser::new(b"%w[foo bar]");
        assert_eq!(parser.parse(), None);
        todo!("implement me");
    }
}
