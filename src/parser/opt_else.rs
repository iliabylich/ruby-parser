use crate::{
    builder::Constructor,
    parser::Parser,
    token::{Token, TokenKind},
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_opt_else(&mut self) -> Option<(Token<'a>, Option<Box<Node<'a>>>)> {
        let else_t = self.try_token(TokenKind::kELSE)?;
        let compstmt = self.try_compstmt();
        Some((else_t, compstmt))
    }
}

#[test]
fn test_opt_else() {
    use crate::parser::RustParser;
    let mut parser = RustParser::new(b"else; 42; end");
    assert_eq!(parser.try_opt_else(), None)
}
