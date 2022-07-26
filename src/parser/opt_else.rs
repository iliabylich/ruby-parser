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
    pub(crate) fn try_opt_else(&mut self) -> ParseResult<(Token, Option<Box<Node>>)> {
        self.all_of("opt else")
            .and(|| self.try_token(TokenKind::kELSE))
            .and(|| self.try_compstmt())
            .unwrap()
    }
}

#[test]
fn test_opt_else() {
    use crate::parser::{ParseError, RustParser};
    let mut parser = RustParser::new(b"else; 42; end");
    assert_eq!(parser.try_opt_else(), Err(ParseError::empty()))
}
