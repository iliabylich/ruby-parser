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
    pub(crate) fn try_words(&mut self) -> Option<Box<Node<'a>>> {
        let begin_t = self.try_token(TokenKind::tWORDS_BEG)?;
        let word_list = self.parse_word_list();
        let end_t = self.expect_token(TokenKind::tSTRING_END);
        Some(Builder::<C>::words_compose(begin_t, word_list, end_t))
    }

    // This rule can be `none
    fn parse_word_list(&mut self) -> Vec<Node<'a>> {
        let mut result = vec![];
        while let Some(word) = self.try_word() {
            result.push(*word)
        }
        result
    }

    pub(crate) fn try_word(&mut self) -> Option<Box<Node<'a>>> {
        let mut contents = vec![];
        while let Some(content) = self.try_string_content() {
            contents.push(*content);
        }
        if contents.is_empty() {
            None
        } else {
            Some(Builder::<C>::word(contents))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{loc::loc, string_content::StringContent, Node, RustParser};

    #[test]
    fn test_words() {
        let mut parser = RustParser::new(b"%W[foo bar]");
        assert_eq!(parser.parse(), None);
        todo!("implement me");
    }
}
