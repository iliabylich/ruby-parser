use crate::{
    builder::Builder,
    parser::{
        base::{ExactToken, ParseResult, Rule},
        trivial::{FnameT, SimpleNumericT},
        Parser,
    },
    Node, TokenKind,
};

pub(crate) struct Literal;
impl Rule for Literal {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

struct Numeric;
impl Rule for Numeric {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tUMINUS_NUM) || SimpleNumericT::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let unary_t = if parser.current_token().is(TokenKind::tUMINUS_NUM) {
            Some(parser.take_token())
        } else {
            None
        };

        type Integer = ExactToken<{ TokenKind::tINTEGER as u8 }>;
        type Float = ExactToken<{ TokenKind::tFLOAT as u8 }>;
        type Rational = ExactToken<{ TokenKind::tRATIONAL as u8 }>;
        type Complex = ExactToken<{ TokenKind::tIMAGINARY as u8 }>;

        let mut number = if parser.current_token().is(TokenKind::tINTEGER) {
            Builder::integer(parser.take_token(), parser.buffer())
        } else if parser.current_token().is(TokenKind::tFLOAT) {
            Builder::float(parser.take_token(), parser.buffer())
        } else if parser.current_token().is(TokenKind::tRATIONAL) {
            Builder::rational(parser.take_token(), parser.buffer())
        } else if parser.current_token().is(TokenKind::tIMAGINARY) {
            Builder::complex(parser.take_token(), parser.buffer())
        } else {
            panic!("wrong token type")
        };

        if let Some(unary_t) = unary_t {
            number = Builder::unary_num(unary_t, number, parser.buffer());
        }

        Ok(number)
    }
}

#[test]
fn test_integer_numeric() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Numeric, b"42", r#"s(:int, "42")"#);
    assert_parses_rule!(Numeric, b"-42", r#"s(:int, "-42")"#);
}

#[test]
fn test_float_numeric() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Numeric, b"42.5", r#"s(:float, "42.5")"#);
    assert_parses_rule!(Numeric, b"-42.5", r#"s(:float, "-42.5")"#);
}

#[test]
fn test_rational_numeric() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Numeric, b"42r", r#"s(:rational, "42r")"#);
    assert_parses_rule!(Numeric, b"-42r", r#"s(:rational, "-42r")"#);
}

#[test]
fn test_complex_numeric() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Numeric, b"42i", r#"s(:complex, "42i")"#);
    assert_parses_rule!(Numeric, b"-42i", r#"s(:complex, "-42i")"#);
}

pub(crate) struct Symbol;
impl Rule for Symbol {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        SimpleSymbol::starts_now(parser) || QuotedSymbol::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if SimpleSymbol::starts_now(parser) {
            SimpleSymbol::parse(parser)
        } else if QuotedSymbol::starts_now(parser) {
            QuotedSymbol::parse(parser)
        } else {
            unreachable!()
        }
    }
}

struct SimpleSymbol;
impl Rule for SimpleSymbol {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tCOLON)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let colon_t = parser.take_token();

        let sym_t = if FnameT::starts_now(parser) {
            FnameT::parse(parser).unwrap()
        } else if parser.current_token().is_one_of([
            TokenKind::tIVAR,
            TokenKind::tCVAR,
            TokenKind::tGVAR,
        ]) {
            parser.take_token()
        } else {
            panic!("wrong token")
        };

        Ok(Builder::symbol(colon_t, sym_t, parser.buffer()))
    }
}

#[test]
fn test_simple_symbol() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(SimpleSymbol, b":foo", r#"s(:sym, "foo")"#);
    assert_parses_rule!(SimpleSymbol, b":@foo", r#"s(:sym, "@foo")"#);
    assert_parses_rule!(SimpleSymbol, b":@@foo", r#"s(:sym, "@@foo")"#);
    assert_parses_rule!(SimpleSymbol, b":$foo", r#"s(:sym, "$foo")"#);
}

struct QuotedSymbol;
impl Rule for QuotedSymbol {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser
            .current_token()
            .is_one_of([TokenKind::tSYMBEG, TokenKind::tDSYMBEG])
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let begin_t = parser.take_token();

        let parts = StringContents::parse(parser).unwrap();

        let end_t = if parser.current_token().is(TokenKind::tSTRING_END) {
            parser.take_token()
        } else {
            panic!("wrong token type")
        };

        Ok(Builder::symbol_compose(begin_t, parts, end_t))
    }
}

struct Strings;

struct String1;

struct Xstring;

struct Regexp;

struct Words;

struct Word;

struct Symbols;

struct Qwords;

struct Qsymbols;

pub(crate) struct StringContents;
impl Rule for StringContents {
    type Output = Vec<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

struct StringContent;
