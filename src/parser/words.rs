use crate::{
    builder::Builder,
    parser::{
        macros::{all_of, at_least_once, maybe, separated_by},
        ParseResult, Parser,
    },
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_words(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, elements, end_t) = all_of!(
            "words",
            self.try_token(TokenKind::tWORDS_BEG),
            parse_word_list(self),
            self.expect_token(TokenKind::tSTRING_END),
        )?;

        Ok(Builder::words_compose(begin_t, elements, end_t))
    }

    pub(crate) fn try_word(&mut self) -> ParseResult<Vec<Node>> {
        at_least_once!("word", self.parse_string_content())
    }
}

// This rule can be `none
fn parse_word_list(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    let word_list = maybe!(separated_by!(
        "word list",
        checkpoint = parser.new_checkpoint(),
        item = parser.try_word().map(|parts| Builder::word(parts)),
        sep = parser.try_token(TokenKind::tSP)
    ))?;

    match word_list {
        Some((word_list, _spaces)) => Ok(word_list),
        None => Ok(vec![]),
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_words_empty() {
        assert_parses!(Parser::parse_words, b"%W[]", "s(:array)")
    }

    #[test]
    fn test_words() {
        assert_parses!(
            Parser::parse_words,
            b"%W[foo bar]",
            r#"
s(:array,
  s(:str, "foo"),
  s(:str, "bar"))
            "#
        )
    }

    #[test]
    fn test_words_interp() {
        assert_parses!(
            Parser::parse_words,
            b"%W[f#{1}oo bar #{42}]",
            r#"
s(:array,
  s(:dstr,
    s(:str, "f"),
    s(:begin,
      s(:int, "1")),
    s(:str, "oo")),
  s(:str, "bar"),
  s(:dstr,
    s(:begin,
      s(:int, "42"))))
            "#
        )
    }
}
