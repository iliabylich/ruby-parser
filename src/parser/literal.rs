use crate::{
    builder::Builder,
    parser::{
        base::{ParseResult, Rule},
        trivial::FnameT,
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

pub(crate) struct Symbol;
impl Rule for Symbol {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        SimpleSymbol::starts_now(parser)
            || SingleQuotedSymbol::starts_now(parser)
            || DoubleQuotedSymbol::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if SimpleSymbol::starts_now(parser) {
            SimpleSymbol::parse(parser)
        } else if SingleQuotedSymbol::starts_now(parser) {
            SingleQuotedSymbol::parse(parser)
        } else if DoubleQuotedSymbol::starts_now(parser) {
            DoubleQuotedSymbol::parse(parser)
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

struct SingleQuotedSymbol;
impl Rule for SingleQuotedSymbol {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tSYMBEG)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

struct DoubleQuotedSymbol;
impl Rule for DoubleQuotedSymbol {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tDSYMBEG)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
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
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

struct StringContent;
