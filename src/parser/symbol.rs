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
    use crate::{
        loc::loc, nodes::Sym, parser::ParseError, string_content::StringContent,
        transactions::assert_err_eq, Node, RustParser,
    };

    #[test]
    fn test_ssym() {
        let mut parser = RustParser::new(b":foo");
        assert_eq!(
            parser.try_ssym(),
            Ok(Box::new(Node::Sym(Sym {
                name: StringContent::from("foo"),
                begin_l: Some(loc!(0, 1)),
                end_l: None,
                expression_l: loc!(0, 4)
            })))
        );
    }

    #[test]
    fn test_ssym_only_colon() {
        let mut parser = RustParser::new(b":");
        assert!(parser.try_ssym().is_err(),);
        // `:` is consumed
        assert_eq!(parser.lexer.buffer().pos(), 1);
    }

    #[test]
    fn test_ssym_no_colon() {
        let mut parser = RustParser::new(b"");
        assert_err_eq!(
            parser.try_ssym(),
            "
ONEOF (1) static symbol
    SEQUENCE (1) :sym (got [])
        TOKEN (1) expected tCOLON, got tEOF (at 0)
    SEQUENCE (1) dynamic symbol value (got [])
        TOKEN (1) expected tSYMBEG, got tEOF (at 0)
"
        );
        assert_eq!(parser.lexer.buffer().pos(), 0);
    }

    #[test]
    fn test_ssym_quoted() {
        let mut parser = RustParser::new(b":'foo'");
        assert_eq!(parser.try_ssym(), Err(ParseError::empty()));
        todo!("implement me");
    }

    #[test]
    fn test_dsym() {
        let mut parser = RustParser::new(b":\"foo\"");
        assert_eq!(parser.try_dsym(), Err(ParseError::empty()));
        todo!("implement me");
    }

    #[test]
    fn test_dsym_only_colon() {
        let mut parser = RustParser::new(b":");
        assert_err_eq!(
            parser.try_dsym(),
            "
SEQUENCE (1) dynamic symbol (got [])
    TOKEN (1) expected tDSYMBEG, got tCOLON (at 0)
"
        );
        // `:` is consumed
        assert_eq!(parser.lexer.buffer().pos(), 1);
    }

    #[test]
    fn test_dsym_no_colon() {
        let mut parser = RustParser::new(b"");
        assert_err_eq!(
            parser.try_dsym(),
            "
SEQUENCE (1) dynamic symbol (got [])
    TOKEN (1) expected tDSYMBEG, got tEOF (at 0)
"
        );
    }
}
