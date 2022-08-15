use crate::{
    builder::Builder,
    parser::{macros::all_of, ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_symbols(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, elements, end_t) = all_of!(
            "symbols",
            self.try_token(TokenKind::tSYMBOLS_BEG),
            parse_symbol_list(self),
            self.expect_token(TokenKind::tSTRING_END),
        )?;

        Ok(Builder::symbols_compose(begin_t, elements, end_t))
    }
}

// This rule can be `none`
fn parse_symbol_list(parser: &mut Parser) -> ParseResult<Vec<Node>> {
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
        assert_parses!(parse_symbols, b"%I[foo bar]", "TODO")
    }
}
