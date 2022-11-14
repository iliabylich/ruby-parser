use crate::{
    builder::Builder,
    parser::base::{Captured, ParseResult, Rule},
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
        let elements = ArrayElements::parse(parser).expect("failed to parse array elements");
        let rbrack_t = if parser.current_token().is(TokenKind::tRBRACK) {
            parser.take_token()
        } else {
            panic!("wrong toke type")
        };

        Ok(Builder::array(Some(lbrack_t), elements, Some(rbrack_t)))
    }
}

struct ArrayElements;
impl Rule for ArrayElements {
    type Output = Vec<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        !parser.current_token().is(TokenKind::tRPAREN)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let mut elements = vec![];
        let mut commas = vec![];

        loop {
            if parser.current_token().is(TokenKind::tRPAREN) {
                break;
            }

            match ArrayElement::parse(parser) {
                Ok(v) => elements.push(*v),
                Err(mut err) => {
                    err.captured = Captured::from(elements) + Captured::from(commas) + err.captured;
                    return Err(err);
                }
            }

            match parser.current_token().kind {
                TokenKind::tCOMMA => commas.push(parser.take_token()),
                TokenKind::tRBRACK => {
                    parser.skip_token();
                    break;
                }
                _ => panic!("wrong token type"),
            }
        }

        // TODO: There must be runtime validations:
        // 1. pairs go after values
        // 2. ',' requires non-empty list of items

        Ok(elements)
    }
}

struct ArrayElement;
impl Rule for ArrayElement {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        SplatElement::starts_now(parser)
            || KeywordSplat::starts_now(parser)
            || LabelToArgPair::starts_now(parser)
            || StringToArgPair::starts_now(parser)
            || ArgToArgPairOrPlainArg::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if SplatElement::starts_now(parser) {
            SplatElement::parse(parser)
        } else if KeywordSplat::starts_now(parser) {
            KeywordSplat::parse(parser)
        } else if LabelToArgPair::starts_now(parser) {
            LabelToArgPair::parse(parser)
        } else if StringToArgPair::starts_now(parser) {
            StringToArgPair::parse(parser)
        } else if ArgToArgPairOrPlainArg::starts_now(parser) {
            ArgToArgPairOrPlainArg::parse(parser)
        } else {
            unreachable!()
        }
    }
}

struct SplatElement;
impl Rule for SplatElement {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tSTAR)
    }

    fn parse(_parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

struct ArgToArgPairOrPlainArg;
impl Rule for ArgToArgPairOrPlainArg {
    type Output = Box<Node>;

    fn starts_now(_parser: &mut Parser) -> bool {
        // Arg::starts_now(parser)
        todo!()
    }

    fn parse(_parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

struct LabelToArgPair;
impl Rule for LabelToArgPair {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tLABEL)
    }

    fn parse(_parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

struct StringToArgPair;
impl Rule for StringToArgPair {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tSTRING_BEG)
    }

    fn parse(_parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

struct KeywordSplat;
impl Rule for KeywordSplat {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tDSTAR)
    }

    fn parse(_parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Array;
    use crate::testing::assert_parses_rule;

    #[test]
    fn test_array_simple() {
        debug_assert!(false, "implement me");
        assert_parses_rule!(
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

    #[test]
    fn test_array_mixed() {
        debug_assert!(false, "implement me");

        assert_parses_rule!(
            Array,
            b"[1, 2, 3, 4 => 5]",
            r#"
s(:array,
  s(:int, "1"),
  s(:int, "2"),
  s(:int, "3"),
  s(:pair,
    s(:int, "4"),
    s(:int, "5")))
            "#
        )
    }
}
