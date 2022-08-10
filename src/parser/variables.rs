use crate::{
    builder::Builder,
    parser::{ParseError, ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn try_gvar(&mut self) -> ParseResult<Box<Node>> {
        let gvar_t = self.try_token(TokenKind::tGVAR)?;
        Ok(Builder::gvar(gvar_t, self.buffer()))
    }

    pub(crate) fn try_back_ref(&mut self) -> ParseResult<Box<Node>> {
        let back_ref_t = self.try_token(TokenKind::tBACK_REF)?;
        Ok(Builder::back_ref(back_ref_t, self.buffer()))
    }

    pub(crate) fn try_nth_ref(&mut self) -> ParseResult<Box<Node>> {
        let nth_ref_t = self.try_token(TokenKind::tNTH_REF)?;
        Ok(Builder::nth_ref(nth_ref_t, self.buffer()))
    }

    pub(crate) fn try_lvar(&mut self) -> ParseResult<Box<Node>> {
        let lvar_t = self.try_token(TokenKind::tIDENTIFIER)?;
        Ok(Builder::lvar(lvar_t, self.buffer()))
    }

    pub(crate) fn try_ivar(&mut self) -> ParseResult<Box<Node>> {
        let ivar_t = self.try_token(TokenKind::tIVAR)?;
        Ok(Builder::ivar(ivar_t, self.buffer()))
    }

    pub(crate) fn try_cvar(&mut self) -> ParseResult<Box<Node>> {
        let cvar_t = self.try_token(TokenKind::tCVAR)?;
        Ok(Builder::cvar(cvar_t, self.buffer()))
    }

    pub(crate) fn try_t_const(&mut self) -> ParseResult<Box<Node>> {
        let const_t = self.try_token(TokenKind::tCONSTANT)?;
        Ok(Builder::const_(const_t, self.buffer()))
    }

    pub(crate) fn try_const_or_identifier(&mut self) -> ParseResult<Token> {
        let token = self.current_token();
        match token.kind {
            TokenKind::tCONSTANT | TokenKind::tIDENTIFIER => {
                self.skip_token();
                Ok(token)
            }
            got => Err(ParseError::OneOfError {
                name: "const or identifier",
                variants: vec![
                    ParseError::TokenError {
                        lookahead: true,
                        expected: TokenKind::tCONSTANT,
                        got,
                        loc: token.loc,
                    },
                    ParseError::TokenError {
                        lookahead: true,
                        expected: TokenKind::tIDENTIFIER,
                        got,
                        loc: token.loc,
                    },
                ],
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_gvar() {
        assert_parses!(try_gvar, b"$foo", "s(:gvar, \"$foo\")");
    }

    #[test]
    fn test_back_ref() {
        assert_parses!(try_back_ref, b"$+", "s(:back_ref, \"$+\")");
    }

    #[test]
    fn test_nth_ref() {
        assert_parses!(try_nth_ref, b"$1", "s(:nth_ref, 1)");
    }

    #[test]
    fn test_lvar() {
        assert_parses!(try_lvar, b"foo", "s(:lvar, \"foo\")");
    }

    #[test]
    fn test_ivar() {
        assert_parses!(try_ivar, b"@foo", "s(:ivar, \"@foo\")");
    }

    #[test]
    fn test_cvar() {
        assert_parses!(try_cvar, b"@@foo", "s(:cvar, \"@@foo\")");
    }

    #[test]
    fn test_const() {
        assert_parses!(try_t_const, b"Foo", "s(:const, nil, \"Foo\")");
    }
}
