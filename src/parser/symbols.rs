use crate::{
    builder::Builder,
    parser::{
        macros::{all_of, maybe, separated_by},
        ParseResult, Parser,
    },
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
    let symbol_list = maybe!(separated_by!(
        "symbol list",
        checkpoint = parser.new_checkpoint(),
        item = parser.try_word().map(|parts| Builder::word(parts)),
        sep = parser.try_token(TokenKind::tSP)
    ))?;

    match symbol_list {
        Some((symbol_list, _spaces)) => Ok(symbol_list),
        None => Ok(vec![]),
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_symbols_empty() {
        assert_parses!(Parser::parse_symbols, b"%I[]", "s(:array)")
    }

    #[test]
    fn test_symbols() {
        assert_parses!(
            Parser::parse_symbols,
            b"%I[foo bar]",
            r#"
s(:array,
  s(:sym, "foo"),
  s(:sym, "bar"))
            "#
        )
    }

    #[test]
    fn test_symbols_interp() {
        assert_parses!(
            Parser::parse_symbols,
            b"%I[f#{1}oo bar #{42}]",
            r#"
s(:array,
  s(:dsym,
    s(:str, "f"),
    s(:begin,
      s(:int, "1")),
    s(:str, "oo")),
  s(:sym, "bar"),
  s(:dsym,
    s(:begin,
      s(:int, "42"))))
            "#
        )
    }
}
