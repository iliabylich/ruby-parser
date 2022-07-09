use crate::{
    builder::Constructor,
    parser::Parser,
    token::{Token, TokenKind},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_opt_ensure(&mut self) -> Option<(Token, Option<Box<Node>>)> {
        let ensure_t = self.try_token(TokenKind::kENSURE)?;
        let compsmt = self.try_compstmt();
        Some((ensure_t, compsmt))
    }
}

#[test]
fn test_opt_ensure() {
    use crate::parser::RustParser;
    let mut parser = RustParser::new(b"ensure; foo; end");
    assert_eq!(parser.try_opt_ensure(), None);
}
