use crate::{
    builder::Builder,
    parser::base::{ParseResult, Rule},
    token::TokenKind,
    Node, Parser,
};

pub(crate) struct Hash;
impl Rule for Hash {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tLCURLY)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

pub(crate) struct Assoc;
impl Rule for Assoc {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

#[test]
fn test_hash() {
    use crate::testing::assert_parses_rule;

    assert_parses_rule!(
        Hash,
        b"{ a: 1, :b => 2, c => 3 }",
        r#"
s(:hash,
  s(:pair,
    s(:sym, "a:"),
    s(:int, "1")),
  s(:pair,
    s(:sym, "b"),
    s(:int, "2")),
  s(:pair,
    s(:lvar, "c"),
    s(:int, "3")))
        "#
    );
}
