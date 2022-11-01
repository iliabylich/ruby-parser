use crate::{
    builder::Builder,
    parser::{
        base::{ExactToken, ParseResult, Rule, SeparatedBy},
        literal::Symbol,
        trivial::FnameT,
        Parser,
    },
    token::TokenKind,
    Node,
};

pub(crate) struct Undef;
impl Rule for Undef {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kUNDEF)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let undef_t = parser.current_token();
        parser.skip_token();

        type CommaTokenRule = ExactToken<{ TokenKind::tCOMMA as u8 }>;

        let names = match SeparatedBy::<Fitem, CommaTokenRule>::parse(parser) {
            Ok((names, _commas)) => names,
            Err(err) => panic!("{:?}", err),
        };

        Ok(Builder::undef(undef_t, names))
    }
}

pub(crate) struct Fitem;

impl Rule for Fitem {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        FnameT::starts_now(parser) || Symbol::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if FnameT::starts_now(parser) {
            let fname_t = FnameT::parse(parser).unwrap();
            Ok(Builder::symbol_internal(fname_t, parser.buffer()))
        } else {
            Symbol::parse(parser)
        }
    }
}

#[test]
fn test_undef() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Undef,
        b"undef a, :b, c",
        r#"
s(:undef,
  s(:sym, "a"),
  s(:sym, "b"),
  s(:sym, "c"))
        "#
    );
}
