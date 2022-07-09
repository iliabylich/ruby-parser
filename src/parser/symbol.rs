use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::TokenKind,
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_symbol(&mut self) -> Option<Box<Node>> {
        self.try_ssym().or_else(|| self.try_dsym())
    }

    fn try_ssym(&mut self) -> Option<Box<Node>> {
        match self.current_token().kind() {
            TokenKind::tCOLON => {
                // maybe a plain symbol
                let checkpoint = self.new_checkpoint();

                let colon_t = self.take_token();
                let sym_t = None
                    .or_else(|| self.try_fname())
                    .or_else(|| self.try_token(TokenKind::tIVAR))
                    .or_else(|| self.try_token(TokenKind::tCVAR))
                    .or_else(|| self.try_token(TokenKind::tGVAR));

                if let Some(sym_t) = sym_t {
                    // definitely a plain symbol
                    let node = Builder::<C>::symbol(colon_t, sym_t, self.buffer());
                    Some(node)
                } else {
                    // rollback
                    self.restore_checkpoint(checkpoint);
                    None
                }
            }
            TokenKind::tSYMBEG => {
                let symbeg_t = self.take_token();
                let contents = self.parse_string_contents();
                let string_end_t = self.expect_token(TokenKind::tSTRING_END);
                let node = Builder::<C>::symbol_compose(symbeg_t, contents, string_end_t);
                Some(node)
            }
            _ => None,
        }
    }

    fn try_dsym(&mut self) -> Option<Box<Node>> {
        let symbeg_t = self.try_token(TokenKind::tDSYMBEG)?;
        let contents = self.parse_string_contents();
        let string_end_t = self.expect_token(TokenKind::tSTRING_END);
        let node = Builder::<C>::symbol_compose(symbeg_t, contents, string_end_t);
        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use crate::{loc::loc, nodes::Sym, string_content::StringContent, Node, RustParser};

    #[test]
    fn test_ssym() {
        let mut parser = RustParser::new(b":foo");
        assert_eq!(
            parser.try_ssym(),
            Some(Box::new(Node::Sym(Sym {
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
        assert_eq!(parser.try_ssym(), None);
        // `:` is consumed
        assert_eq!(parser.lexer.buffer.pos(), 1);
    }

    #[test]
    fn test_ssym_no_colon() {
        let mut parser = RustParser::new(b"");
        assert_eq!(parser.try_ssym(), None);
        assert_eq!(parser.lexer.buffer.pos(), 0);
    }

    #[test]
    fn test_ssym_quoted() {
        let mut parser = RustParser::new(b":'foo'");
        assert_eq!(parser.try_ssym(), None);
        todo!("implement me");
    }

    #[test]
    fn test_dsym() {
        let mut parser = RustParser::new(b":\"foo\"");
        assert_eq!(parser.try_dsym(), None);
        todo!("implement me");
    }

    #[test]
    fn test_dsym_only_colon() {
        let mut parser = RustParser::new(b":");
        assert_eq!(parser.try_dsym(), None);
        // `:` is consumed
        assert_eq!(parser.lexer.buffer.pos(), 1);
    }

    #[test]
    fn test_dsym_no_colon() {
        let mut parser = RustParser::new(b"");
        assert_eq!(parser.try_dsym(), None);
        assert_eq!(parser.lexer.buffer.pos(), 0);
    }
}
