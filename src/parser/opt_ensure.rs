use crate::{
    builder::Constructor,
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_opt_ensure(&mut self) -> ParseResult<(Token, Option<Box<Node>>)> {
        self.all_of("opt ensure")
            .and(|| self.try_token(TokenKind::kENSURE))
            .and(|| self.try_compstmt())
            .unwrap()
    }
}

#[test]
fn test_opt_ensure() {
    use crate::parser::{ParseError, RustParser};
    let mut parser = RustParser::new(b"ensure; foo; end");
    assert_eq!(parser.try_opt_ensure(), Err(ParseError::empty()));
}
