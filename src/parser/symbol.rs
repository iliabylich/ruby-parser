use crate::{
    builder::{Builder, Constructor},
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_symbol(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("symbol")
            .or_else(|| self.try_ssym())
            .or_else(|| self.try_dsym())
            .compact()
            .stop()
    }

    fn try_ssym(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("static symbol")
            .or_else(|| {
                let (colon_t, sym_t) = self
                    .all_of(":sym")
                    .and(|| self.try_token(TokenKind::tCOLON))
                    .and(|| {
                        self.one_of("static symbol value")
                            .or_else(|| self.try_fname())
                            .or_else(|| self.try_token(TokenKind::tIVAR))
                            .or_else(|| self.try_token(TokenKind::tCVAR))
                            .or_else(|| self.try_token(TokenKind::tGVAR))
                            .required()
                            .stop()
                    })
                    .stop()?;

                Ok(Builder::<C>::symbol(colon_t, sym_t, self.buffer()))
            })
            .or_else(|| {
                let (begin_t, parts, end_t) = self
                    .all_of("dynamic symbol value")
                    .and(|| self.try_token(TokenKind::tSYMBEG))
                    .and(|| self.try_string_contents())
                    .and(|| self.expect_token(TokenKind::tSTRING_END))
                    .stop()?;

                Ok(Builder::<C>::symbol_compose(begin_t, parts, end_t))
            })
            .compact()
            .stop()
    }

    fn try_dsym(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, parts, end_t) = self
            .all_of("dynamic symbol")
            .and(|| self.try_token(TokenKind::tDSYMBEG))
            .and(|| self.try_string_contents())
            .and(|| self.expect_token(TokenKind::tSTRING_END))
            .stop()?;

        let node = Builder::<C>::symbol_compose(begin_t, parts, end_t);
        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use crate::{testing::assert_parses, testing::assert_parses_with_error};

    #[test]
    fn test_ssym() {
        assert_parses!(try_ssym, b":foo", "s(:sym, \"foo\")")
    }

    #[test]
    fn test_ssym_only_colon() {
        let parser = assert_parses_with_error!(try_ssym, b":");
        // `:` is consumed
        assert_eq!(parser.lexer.buffer().pos(), 1);
    }

    #[test]
    fn test_ssym_no_colon() {
        let parser = assert_parses_with_error!(
            try_ssym,
            b"",
            "
ONEOF (0) static symbol
    SEQUENCE (0) :sym (got [])
        TOKEN (0) expected tCOLON, got tEOF (at 0)
    SEQUENCE (0) dynamic symbol value (got [])
        TOKEN (0) expected tSYMBEG, got tEOF (at 0)
"
        );
        assert_eq!(parser.lexer.buffer().pos(), 0);
    }

    #[test]
    fn test_ssym_quoted() {
        assert_parses!(try_ssym, b":'foo'", "TODO")
    }

    #[test]
    fn test_dsym() {
        assert_parses!(try_dsym, b":\"foo\"", "TODO")
    }

    #[test]
    fn test_dsym_only_colon() {
        let parser = assert_parses_with_error!(
            try_dsym,
            b":",
            "
SEQUENCE (0) dynamic symbol (got [])
    TOKEN (0) expected tDSYMBEG, got tCOLON (at 0)
"
        );
        // `:` is consumed
        assert_eq!(parser.lexer.buffer().pos(), 1);
    }

    #[test]
    fn test_dsym_no_colon() {
        assert_parses_with_error!(
            try_dsym,
            b"",
            "
SEQUENCE (0) dynamic symbol (got [])
    TOKEN (0) expected tDSYMBEG, got tEOF (at 0)
"
        );
    }
}
