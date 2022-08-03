use crate::{
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn try_opt_ensure(&mut self) -> ParseResult<(Token, Option<Box<Node>>)> {
        self.all_of("opt ensure")
            .and(|| self.try_token(TokenKind::kENSURE))
            .and(|| self.try_compstmt())
            .stop()
    }
}

#[test]
fn test_opt_ensure() {
    use crate::{loc::loc, nodes::Int, parser::Parser, string_content::StringContent, Node};
    let mut parser = Parser::new(b"ensure 42 end");
    assert_eq!(
        parser.try_opt_ensure(),
        Ok((
            Token {
                kind: TokenKind::kENSURE,
                loc: loc!(0, 6),
                value: None
            },
            Some(Box::new(Node::Int(Int {
                value: StringContent::from("42"),
                operator_l: None,
                expression_l: loc!(7, 9)
            })))
        ))
    );
}
