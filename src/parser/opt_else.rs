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
            .stop()
    }
}

#[test]
fn test_opt_else() {
    use crate::{
        loc::loc, nodes::Int, parser::RustParser, string_content::StringContent, token::token, Node,
    };
    let mut parser = RustParser::new(b"else 42 end").debug();
    assert_eq!(
        parser.try_opt_else(),
        Ok((
            token!(kELSE, loc!(0, 4)),
            Some(Box::new(Node::Int(Int {
                value: StringContent::from("42"),
                operator_l: None,
                expression_l: loc!(5, 7)
            })))
        ))
    )
}
