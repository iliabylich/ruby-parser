use crate::{
    builder::Builder,
    parser::{
        base::{ParseResult, Rule},
        ParseResult as ParseResult2, Parser,
    },
    token::{Token, TokenKind},
    Node,
};

use super::macros::one_of;

pub(crate) struct Gvar;
impl Rule for Gvar {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tGVAR)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let gvar_t = parser.take_token();
        Ok(Builder::gvar(gvar_t, parser.buffer()))
    }
}

pub(crate) struct Ivar;
impl Rule for Ivar {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tIVAR)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let ivar_t = parser.take_token();
        Ok(Builder::ivar(ivar_t, parser.buffer()))
    }
}

pub(crate) struct Cvar;
impl Rule for Cvar {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tCVAR)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let cvar_t = parser.take_token();
        Ok(Builder::cvar(cvar_t, parser.buffer()))
    }
}

pub(crate) struct BackRef;
impl Rule for BackRef {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tBACK_REF)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let back_ref_t = parser.take_token();
        Ok(Builder::back_ref(back_ref_t, parser.buffer()))
    }
}

pub(crate) struct NthRef;
impl Rule for NthRef {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tNTH_REF)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let nth_ref_t = parser.take_token();
        Ok(Builder::nth_ref(nth_ref_t, parser.buffer()))
    }
}

impl Parser {
    pub(crate) fn parse_gvar(&mut self) -> ParseResult2<Box<Node>> {
        let gvar_t = self.try_token(TokenKind::tGVAR)?;
        Ok(Builder::gvar(gvar_t, self.buffer()))
    }

    pub(crate) fn parse_back_ref(&mut self) -> ParseResult2<Box<Node>> {
        let back_ref_t = self.try_token(TokenKind::tBACK_REF)?;
        Ok(Builder::back_ref(back_ref_t, self.buffer()))
    }

    pub(crate) fn parse_nth_ref(&mut self) -> ParseResult2<Box<Node>> {
        let nth_ref_t = self.try_token(TokenKind::tNTH_REF)?;
        Ok(Builder::nth_ref(nth_ref_t, self.buffer()))
    }

    pub(crate) fn parse_lvar(&mut self) -> ParseResult2<Box<Node>> {
        let lvar_t = self.try_token(TokenKind::tIDENTIFIER)?;
        Ok(Builder::lvar(lvar_t, self.buffer()))
    }

    pub(crate) fn parse_ivar(&mut self) -> ParseResult2<Box<Node>> {
        let ivar_t = self.try_token(TokenKind::tIVAR)?;
        Ok(Builder::ivar(ivar_t, self.buffer()))
    }

    pub(crate) fn parse_cvar(&mut self) -> ParseResult2<Box<Node>> {
        let cvar_t = self.try_token(TokenKind::tCVAR)?;
        Ok(Builder::cvar(cvar_t, self.buffer()))
    }

    pub(crate) fn parse_t_const(&mut self) -> ParseResult2<Box<Node>> {
        let const_t = self.try_token(TokenKind::tCONSTANT)?;
        Ok(Builder::const_(const_t, self.buffer()))
    }

    pub(crate) fn parse_const_or_identifier(&mut self) -> ParseResult2<Token> {
        one_of!(
            "const or identifier",
            self.try_token(TokenKind::tIDENTIFIER),
            self.try_token(TokenKind::tCONSTANT),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_gvar() {
        assert_parses!(Parser::parse_gvar, b"$foo", "s(:gvar, \"$foo\")");
    }

    #[test]
    fn test_back_ref() {
        assert_parses!(Parser::parse_back_ref, b"$+", "s(:back_ref, \"$+\")");
    }

    #[test]
    fn test_nth_ref() {
        assert_parses!(Parser::parse_nth_ref, b"$1", "s(:nth_ref, 1)");
    }

    #[test]
    fn test_lvar() {
        assert_parses!(Parser::parse_lvar, b"foo", "s(:lvar, \"foo\")");
    }

    #[test]
    fn test_ivar() {
        assert_parses!(Parser::parse_ivar, b"@foo", "s(:ivar, \"@foo\")");
    }

    #[test]
    fn test_cvar() {
        assert_parses!(Parser::parse_cvar, b"@@foo", "s(:cvar, \"@@foo\")");
    }

    #[test]
    fn test_const() {
        assert_parses!(Parser::parse_t_const, b"Foo", "s(:const, nil, \"Foo\")");
    }
}
