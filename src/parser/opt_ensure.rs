use crate::{
    builder::Constructor,
    parser::{ParseError, Parser},
    token::{Token, TokenKind},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_opt_ensure(&mut self) -> Result<(Token, Option<Box<Node>>), ParseError> {
        let ensure_t = self.try_token(TokenKind::kENSURE)?;
        let compsmt = self.try_compstmt()?;
        Ok((ensure_t, compsmt))
    }
}

#[test]
fn test_opt_ensure() {
    use crate::parser::RustParser;
    let mut parser = RustParser::new(b"ensure; foo; end");
    assert_eq!(parser.try_opt_ensure(), Err(ParseError::empty()));
}
