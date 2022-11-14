use crate::{
    builder::Builder,
    parser::{ParseResult, Rule},
    token::{Token, TokenKind},
    Node, Parser,
};

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

pub(crate) struct Lvar;
impl Rule for Lvar {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tIDENTIFIER)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let lvar_t = parser.take_token();
        Ok(Builder::lvar(lvar_t, parser.buffer()))
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

#[cfg(test)]
mod tests {
    use super::{BackRef, Cvar, Gvar, Ivar, Lvar, NthRef};
    use crate::testing::assert_parses_rule;

    #[test]
    fn test_gvar() {
        assert_parses_rule!(Gvar, b"$foo", "s(:gvar, \"$foo\")");
    }

    #[test]
    fn test_back_ref() {
        assert_parses_rule!(BackRef, b"$+", "s(:back_ref, \"$+\")");
    }

    #[test]
    fn test_nth_ref() {
        assert_parses_rule!(NthRef, b"$1", "s(:nth_ref, 1)");
    }

    #[test]
    fn test_lvar() {
        assert_parses_rule!(Lvar, b"foo", "s(:lvar, \"foo\")");
    }

    #[test]
    fn test_ivar() {
        assert_parses_rule!(Ivar, b"@foo", "s(:ivar, \"@foo\")");
    }

    #[test]
    fn test_cvar() {
        assert_parses_rule!(Cvar, b"@@foo", "s(:cvar, \"@@foo\")");
    }
}
