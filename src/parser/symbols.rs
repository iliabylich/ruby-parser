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
        let (begin_t, elements, end_t) = self
            .all_of("symbols")
            .and(|| self.try_token(TokenKind::tSYMBOLS_BEG))
            .and(|| parse_symbol_list(self))
            .and(|| self.expect_token(TokenKind::tSTRING_END))
            .unwrap()?;

        Ok(Builder::<C>::symbols_compose(begin_t, elements, end_t))
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
