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
    pub(crate) fn try_words(&mut self) -> Result<Box<Node>, ParseError> {
        let begin_t = self.try_token(TokenKind::tWORDS_BEG)?;
        let word_list = self.parse_word_list()?;
        let end_t = self.expect_token(TokenKind::tSTRING_END);
        Ok(Builder::<C>::words_compose(begin_t, word_list, end_t))
    }

    // This rule can be `none
    fn parse_word_list(&mut self) -> Result<Vec<Node>, ParseError> {
        let mut result = vec![];
        while let Some(word) = self.try_word()? {
            result.push(*word)
        }
        Ok(result)
    }

    pub(crate) fn try_word(&mut self) -> Result<Option<Box<Node>>, ParseError> {
        let mut contents = self.parse_string_contents()?;
        if contents.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::<C>::word(contents)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{loc::loc, parser::ParseError, string_content::StringContent, Node, RustParser};

    #[test]
    fn test_words() {
        let mut parser = RustParser::new(b"%W[foo bar]");
        assert_eq!(parser.try_words(), Err(ParseError::empty()));
        todo!("implement me");
    }
}
