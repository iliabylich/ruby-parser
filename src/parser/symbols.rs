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
    pub(crate) fn try_symbols(&mut self) -> Result<Box<Node>, ParseError> {
        let begin_t = self.try_token(TokenKind::tSYMBOLS_BEG)?;
        let word_list = parse_symbol_list(self)?;
        let end_t = self.expect_token(TokenKind::tSTRING_END);
        Ok(Builder::<C>::symbols_compose(begin_t, word_list, end_t))
    }
}

// This rule can be `none`
fn parse_symbol_list<C: Constructor>(parser: &mut Parser<C>) -> Result<Vec<Node>, ParseError> {
    let mut result = vec![];
    while let Some(word) = parser.try_word()? {
        result.push(*word);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{loc::loc, parser::ParseError, string_content::StringContent, Node, RustParser};

    #[test]
    fn test_symbols() {
        let mut parser = RustParser::new(b"%I[foo bar]");
        assert_eq!(parser.try_symbols(), Err(ParseError::empty()));
        todo!("implement me");
    }
}
