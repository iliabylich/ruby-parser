use crate::{
    builder::Builder,
    parser::{
        base::{at_most_one_is_true, ExactToken, ParseResult, Rule, SeparatedBy},
        Value,
    },
    Node, Parser, TokenKind,
};

pub(crate) struct Params;
impl Rule for Params {
    type Output = Vec<Node>;

    fn starts_now(_parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        type CommaT = ExactToken<{ TokenKind::tCOMMA as u8 }>;
        type R = SeparatedBy<Param, CommaT>;

        let (args, _commas) = R::parse(parser).unwrap();
        // TODO: There must be runtime validations:
        // 1. params are ordered
        //    req -> opt -> (single) rest -> post -> kw[req/opt/rest] -> block

        Ok(args)
    }
}

struct Param;
impl Rule for Param {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        let token = parser.current_token();

        at_most_one_is_true([
            Restarg::starts_now(parser),
            Kwrestarg::starts_now(parser),
            Blockarg::starts_now(parser),
            ParenthesizedMultiArg::starts_now(parser),
            token.is(TokenKind::tIDENTIFIER),
            token.is(TokenKind::tLABEL),
        ])
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if Restarg::starts_now(parser) {
            Restarg::parse(parser)
        } else if Kwrestarg::starts_now(parser) {
            Kwrestarg::parse(parser)
        } else if Blockarg::starts_now(parser) {
            Blockarg::parse(parser)
        } else if ParenthesizedMultiArg::starts_now(parser) {
            ParenthesizedMultiArg::parse(parser)
        } else if parser.current_token().is(TokenKind::tIDENTIFIER) {
            let name_t = parser.take_token();
            if parser.current_token().is(TokenKind::tEQL) {
                // (a = 1)
                let eql_t = parser.take_token();
                let default = Value::parse(parser).unwrap();
                Ok(Builder::optarg(name_t, eql_t, default, parser.buffer()))
            } else {
                // just (a)
                Ok(Builder::arg(name_t, parser.buffer()))
            }
        } else if parser.current_token().is(TokenKind::tLABEL) {
            let name_t = parser.take_token();
            if Value::starts_now(parser) {
                // (a: 1)
                let default = Value::parse(parser).unwrap();
                Ok(Builder::kwoptarg(name_t, default, parser.buffer()))
            } else {
                // just (a:)
                Ok(Builder::kwarg(name_t, parser.buffer()))
            }
        } else {
            unreachable!()
        }
    }
}
#[test]
fn test_arg() {
    crate::testing::assert_parses_rule!(Param, b"a", "s(:arg, \"a\")")
}
#[test]
fn test_optarg() {
    crate::testing::assert_parses_rule!(
        Param,
        b"a = 42",
        r#"
s(:optarg, "a",
  s(:int, "42"))
        "#
    )
}
#[test]
fn test_restarg() {
    crate::testing::assert_parses_rule!(Param, b"*a", "s(:restarg, \"a\")")
}
#[test]
fn test_kwarg() {
    crate::testing::assert_parses_rule!(Param, b"a:", "s(:kwarg, \"a\")")
}
#[test]
fn test_kwoptarg() {
    crate::testing::assert_parses_rule!(
        Param,
        b"a: 42",
        r#"
s(:kwoptarg, "a",
  s(:int, "42"))
        "#
    )
}
#[test]
fn test_kwrestarg() {
    crate::testing::assert_parses_rule!(Param, b"**a", "s(:kwrestarg, \"a\")")
}
#[test]
fn test_blockarg() {
    crate::testing::assert_parses_rule!(Param, b"&a", "s(:blockarg, \"a\")")
}
#[test]
fn test_multi_arg() {
    crate::testing::assert_parses_rule!(
        Param,
        b"(a, *b, (c, d))",
        r#"
s(:mlhs,
  s(:arg, "a"),
  s(:restarg, "b"),
  s(:mlhs,
    s(:arg, "c"),
    s(:arg, "d")))
        "#
    )
}

struct Restarg;
impl Rule for Restarg {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tSTAR)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let star_t = parser.take_token();
        let name_t = if parser.current_token().is(TokenKind::tIDENTIFIER) {
            Some(parser.take_token())
        } else {
            None
        };
        Ok(Builder::restarg(star_t, name_t, parser.buffer()))
    }
}

struct Kwrestarg;
impl Rule for Kwrestarg {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tDSTAR)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let dstar_t = parser.take_token();
        let name_t = if parser.current_token().is(TokenKind::tIDENTIFIER) {
            Some(parser.take_token())
        } else {
            None
        };
        Ok(Builder::kwrestarg(dstar_t, name_t, parser.buffer()))
    }
}

struct Blockarg;
impl Rule for Blockarg {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tAMPER)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let amper_t = parser.take_token();
        let name_t = parser.expect_token(TokenKind::tIDENTIFIER).unwrap();
        Ok(Builder::blockarg(amper_t, Some(name_t), parser.buffer()))
    }
}

struct ParenthesizedMultiArg;
impl Rule for ParenthesizedMultiArg {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tLPAREN)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let begin_t = parser.take_token();
        let items = MultiArgs::parse(parser).unwrap();
        let end_t = parser.expect_token(TokenKind::tRPAREN).unwrap();

        // TODO: move to builder
        use crate::nodes::Mlhs;
        let begin_l = begin_t.loc;
        let end_l = end_t.loc;
        Ok(Box::new(Node::Mlhs(Mlhs {
            items,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l: begin_l.join(&end_l),
        })))
    }
}

struct MultiArgs;
impl Rule for MultiArgs {
    type Output = Vec<Node>;

    fn starts_now(_parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        type CommaT = ExactToken<{ TokenKind::tCOMMA as u8 }>;
        type R = SeparatedBy<MultiArg, CommaT>;

        let (args, _commas) = R::parse(parser).unwrap();
        Ok(args)
    }
}

struct MultiArg;
impl Rule for MultiArg {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        at_most_one_is_true([
            parser.current_token().is(TokenKind::tIDENTIFIER),
            Restarg::starts_now(parser),
            ParenthesizedMultiArg::starts_now(parser),
        ])
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if parser.current_token().is(TokenKind::tIDENTIFIER) {
            let name_t = parser.take_token();
            Ok(Builder::arg(name_t, parser.buffer()))
        } else if Restarg::starts_now(parser) {
            Restarg::parse(parser)
        } else if ParenthesizedMultiArg::starts_now(parser) {
            ParenthesizedMultiArg::parse(parser)
        } else {
            unreachable!()
        }
    }
}
