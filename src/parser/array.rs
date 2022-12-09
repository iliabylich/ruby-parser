use crate::{
    builder::Builder,
    parser::{
        base::{at_most_one_is_true, ExactToken, Maybe1, ParseResult, Rule, SeparatedBy},
        Value,
    },
    token::TokenKind,
    Node, Parser,
};

pub(crate) struct Array;
impl Rule for Array {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tLBRACK)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let lbrack_t = parser.take_token();
        let elements = Items::parse(parser).expect("failed to parse array elements");
        let rbrack_t = if parser.current_token().is(TokenKind::tRBRACK) {
            parser.take_token()
        } else {
            panic!("wrong toke type")
        };

        Ok(Builder::array(Some(lbrack_t), elements, Some(rbrack_t)))
    }
}
#[test]
fn test_array_simple() {
    crate::testing::assert_parses_rule!(
        Array,
        b"[1, 2, 3]",
        r#"
s(:array,
  s(:int, "1"),
  s(:int, "2"),
  s(:int, "3"))
        "#
    )
}

struct Items;
impl Rule for Items {
    type Output = Vec<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        !parser.current_token().is(TokenKind::tRPAREN)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        type CommaT = ExactToken<{ TokenKind::tCOMMA as u8 }>;

        let (items, _commas) = SeparatedBy::<Item, CommaT>::parse(parser).unwrap();
        let _trailing_comma = Maybe1::<CommaT>::parse(parser).unwrap();

        // TODO: There must be runtime validations:
        // 1. pairs go after values
        // 2. ',' requires non-empty list of items

        Ok(items)
    }
}

struct Item;
impl Rule for Item {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        at_most_one_is_true([
            SplatElement::starts_now(parser),
            KeywordSplat::starts_now(parser),
            LabelToValuePair::starts_now(parser),
            Value::starts_now(parser), // StringToValuePair::starts_now(parser),
                                       // ValueToValuePairOrPlainValue::starts_now(parser),
        ])
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if SplatElement::starts_now(parser) {
            SplatElement::parse(parser)
        } else if KeywordSplat::starts_now(parser) {
            KeywordSplat::parse(parser)
        } else if LabelToValuePair::starts_now(parser) {
            LabelToValuePair::parse(parser)
        } else if Value::starts_now(parser) {
            let value = Value::parse(parser).unwrap();
            if matches!(&*value, Node::Str(_)) && parser.current_token().is(TokenKind::tCOLON) {
                // "foo": value
                let key = value;
                let colon_t = parser.take_token();
                let value = Value::parse(parser).unwrap();
                Ok(Builder::pair_quoted(key, colon_t, value))
            } else if parser.current_token().is(TokenKind::tASSOC) {
                // pair `value => value`
                let key = value;
                let assoc_t = parser.take_token();
                let value = Value::parse(parser).unwrap();
                Ok(Builder::pair(key, assoc_t, value))
            } else {
                // just value
                Ok(value)
            }
        } else {
            unreachable!()
        }
    }
}
#[test]
fn test_value_to_value() {
    crate::testing::assert_parses_rule!(
        Item,
        b"42 => 42",
        r#"
s(:pair,
  s(:int, "42"),
  s(:int, "42"))
        "#
    )
}
#[test]
fn test_quoted_label_pair() {
    crate::testing::assert_parses_rule!(
        Item,
        b"\"foo\": 42",
        r#"
s(:pair,
  s(:sym, "foo"),
  s(:int, "42"))
        "#
    )
}
#[test]
fn test_element() {
    crate::testing::assert_parses_rule!(Item, b"42", r#"s(:int, "42")"#)
}

struct SplatElement;
impl Rule for SplatElement {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tSTAR)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let star_t = parser.take_token();
        let value = Value::parse(parser).unwrap();
        Ok(Builder::splat(star_t, value))
    }
}
#[test]
fn test_splat_element() {
    crate::testing::assert_parses_rule!(
        Item,
        b"*42",
        r#"
s(:splat,
  s(:int, "42"))
        "#
    )
}

struct LabelToValuePair;
impl Rule for LabelToValuePair {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tLABEL)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let key_t = parser.take_token();
        let value = Maybe1::<Value>::parse(parser).unwrap();
        if let Some(value) = value {
            Ok(Builder::pair_keyword(key_t, value, parser.buffer()))
        } else {
            Ok(Builder::pair_label(key_t, parser.buffer()))
        }
    }
}
#[test]
fn test_label_pair() {
    crate::testing::assert_parses_rule!(
        Item,
        b"foo: 42",
        r#"
s(:pair,
  s(:sym, "foo"),
  s(:int, "42"))
        "#
    )
}
#[test]
fn test_label_without_value() {
    crate::testing::assert_parses_rule!(
        Item,
        b"foo:",
        r#"
s(:pair,
  s(:sym, "foo"),
  s(:lvar, "foo"))
        "#
    )
}

struct KeywordSplat;
impl Rule for KeywordSplat {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tDSTAR)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let dstar_t = parser.take_token();
        let value = Value::parse(parser).unwrap();
        Ok(Builder::kwsplat(dstar_t, value))
    }
}
#[test]
fn test_kwsplat_element() {
    crate::testing::assert_parses_rule!(
        Item,
        b"**42",
        r#"
s(:kwsplat,
  s(:int, "42"))
        "#
    )
}
