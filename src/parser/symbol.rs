use crate::{
    builder::{Builder, Constructor},
    parser::{ParseError, Parser},
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_symbol(&mut self) -> Result<Box<Node>, ParseError> {
        self.chain("symbol")
            .or_else(|| self.try_ssym())
            .or_else(|| self.try_dsym())
            .done()
    }

    fn try_ssym(&mut self) -> Result<Box<Node>, ParseError> {
        self.chain("static symbol")
            .or_else(|| {
                let colon_t = self.try_token(TokenKind::tCOLON)?;
                let sym_t = self
                    .chain("static symbol value")
                    .or_else(|| self.try_fname())
                    .or_else(|| self.try_token(TokenKind::tIVAR))
                    .or_else(|| self.try_token(TokenKind::tCVAR))
                    .or_else(|| self.try_token(TokenKind::tGVAR))
                    .required()
                    .strip()
                    .done()?;
                Ok(Builder::<C>::symbol(colon_t, sym_t, self.buffer()))
            })
            .or_else(|| {
                let symbeg_t = self.try_token(TokenKind::tSYMBEG)?;
                let contents = self.parse_string_contents()?;
                let string_end_t = self.expect_token(TokenKind::tSTRING_END);
                Ok(Builder::<C>::symbol_compose(
                    symbeg_t,
                    contents,
                    string_end_t,
                ))
            })
            .strip()
            .done()
    }

    fn try_dsym(&mut self) -> Result<Box<Node>, ParseError> {
        let symbeg_t = self.try_token(TokenKind::tDSYMBEG)?;
        let contents = self.parse_string_contents()?;
        let string_end_t = self.expect_token(TokenKind::tSTRING_END);
        let node = Builder::<C>::symbol_compose(symbeg_t, contents, string_end_t);
        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        loc::loc, nodes::Sym, parser::ParseError, string_content::StringContent, token::TokenKind,
        Node, RustParser,
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
        assert_eq!(
            parser.try_ssym(),
            Err(ParseError::expected_rule("static symbol", TokenKind::tEOF))
        );
        // `:` is consumed
        assert_eq!(parser.lexer.buffer().pos(), 1);
    }

    #[test]
    fn test_ssym_no_colon() {
        let mut parser = RustParser::new(b"");
        assert_eq!(parser.try_ssym(), Err(ParseError::empty()));
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
        assert_eq!(parser.try_dsym(), Err(ParseError::empty()));
        // `:` is consumed
        assert_eq!(parser.lexer.buffer().pos(), 1);
    }

    #[test]
    fn test_dsym_no_colon() {
        let mut parser = RustParser::new(b"");
        assert_eq!(parser.try_dsym(), Err(ParseError::empty()));
        assert_eq!(parser.lexer.buffer().pos(), 0);
    }
}
