use crate::{
    builder::Builder,
    parser::{macros::all_of, ParseResult, Parser},
    token::TokenKind,
    Node,
};

use super::macros::separated_by::separated_by;

impl Parser {
    pub(crate) fn parse_undef(&mut self) -> ParseResult<Box<Node>> {
        let (undef_t, names) = all_of!(
            "undef ...",
            self.try_token(TokenKind::kUNDEF),
            parse_names(self),
        )?;

        Ok(Builder::undef(undef_t, names))
    }
}

fn parse_names(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    let (names, _commas) = separated_by!(
        "undef named",
        checkpoint = parser.new_checkpoint(),
        item = parser.parse_fitem(),
        sep = parser.try_token(TokenKind::tCOMMA)
    )?;

    Ok(names)
}

#[test]
fn test_undef() {
    use crate::testing::assert_parses;
    assert_parses!(
        parse_undef,
        b"undef a, :b, c",
        r#"
s(:undef,
  s(:sym, "a"),
  s(:sym, "b"),
  s(:sym, "c"))
        "#
    );
}
