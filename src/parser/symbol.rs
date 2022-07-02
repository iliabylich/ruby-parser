use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::TokenValue,
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_symbol(&mut self) -> Option<Box<Node<'a>>> {
        let checkpoint = self.new_checkpoint();

        match self.current_token().value() {
            TokenValue::tCOLON => {
                // maybe a plain symbol
                let colon_t = self.take_token();
                let sym_t = None
                    .or_else(|| self.try_fname())
                    .or_else(|| self.try_token(TokenValue::tIVAR))
                    .or_else(|| self.try_token(TokenValue::tCVAR))
                    .or_else(|| self.try_token(TokenValue::tGVAR));

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
            TokenValue::tSYMBEG | TokenValue::tDSYMBEG => {
                let symbeg_t = self.take_token();
                let contents = self.parse_string_contents();
                let string_end_t = self.expect_token(TokenValue::tSTRING_END);
                let node = Builder::<C>::symbol_compose(symbeg_t, contents, string_end_t);
                Some(node)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{loc::loc, nodes::Sym, string_content::StringContent, Node, RustParser};

    #[test]
    fn test_symbol_plain() {
        let mut parser = RustParser::new(b":foo");
        assert_eq!(
            parser.try_symbol(),
            Some(Box::new(Node::Sym(Sym {
                name: StringContent::from("foo"),
                begin_l: Some(loc!(0, 1)),
                end_l: None,
                expression_l: loc!(0, 4)
            })))
        );
    }

    #[test]
    fn test_sym_squoted() {
        let _parser = RustParser::new(b":'foo'");
        unimplemented!("requires parse_string_contents");
    }

    #[test]
    fn test_sym_dquoted() {
        let _parser = RustParser::new(b":\"foo\"");
        unimplemented!("requires parse_string_contents");
    }

    #[test]
    fn test_only_colon() {
        let mut parser = RustParser::new(b":");
        assert_eq!(parser.try_symbol(), None);
        assert_eq!(parser.lexer.buffer.pos(), 0);
    }

    #[test]
    fn test_no_colon() {
        let mut parser = RustParser::new(b"");
        assert_eq!(parser.try_symbol(), None);
        assert_eq!(parser.lexer.buffer.pos(), 0);
    }
}
