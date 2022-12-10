use crate::{
    builder::Builder,
    parser::{
        base::{Maybe1, ParseResult, Repeat1, Rule},
        Compstmt, Mrhs, Then, Value,
    },
    Node, Parser, Token, TokenKind,
};

pub(crate) type OptRescue = Repeat1<Rescue>;

pub(crate) struct Rescue;
impl Rule for Rescue {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kRESCUE)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let rescue_t = parser.take_token();
        let exc_list = ExcList::parse(parser).unwrap();
        let assoc_t_and_exc_var = Maybe1::<ExcVar>::parse(parser).unwrap();
        let then_t = Then::parse(parser).unwrap();
        let body = Compstmt::parse(parser).unwrap();
        Ok(Builder::rescue_body(
            rescue_t,
            exc_list,
            assoc_t_and_exc_var,
            then_t,
            body,
        ))
    }
}
#[test]
fn test_rescue() {
    crate::testing::assert_parses_rule!(
        Rescue,
        b"rescue Foo => e2; 'body2'",
        r#"
s(:resbody,
  s(:array,
    s(:const, nil, "Foo")),
  s(:lvar, "e2"),
  s(:str, "body2"))
        "#
    )
}

struct ExcList;
impl Rule for ExcList {
    type Output = Vec<Node>;

    fn starts_now(_parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if Mrhs::starts_now(parser) {
            Mrhs::parse(parser)
        } else {
            Ok(vec![])
        }
    }
}

struct ExcVar;
impl Rule for ExcVar {
    type Output = (Token, Box<Node>);

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tASSOC)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let assoc_t = parser.take_token();
        let value = Value::parse(parser).unwrap();
        Ok((assoc_t, value))
    }
}
