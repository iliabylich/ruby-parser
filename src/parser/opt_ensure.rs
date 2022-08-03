use crate::{
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

type Ensure = (Token, Option<Box<Node>>);

impl Parser {
    pub(crate) fn try_opt_ensure(&mut self) -> ParseResult<Option<Ensure>> {
        self.one_of("opt ensure")
            .or_else(|| parse_ensure(self).map(|v| Some(v)))
            .or_else(|| Ok(None))
            .stop()
    }
}

fn parse_ensure(parser: &mut Parser) -> ParseResult<Ensure> {
    parser
        .all_of("ensure")
        .and(|| parser.try_token(TokenKind::kENSURE))
        .and(|| parser.try_compstmt())
        .stop()
}

#[test]
fn test_opt_ensure() {
    use crate::{loc::loc, nodes::Int, parser::Parser, string_content::StringContent, Node};
    let mut parser = Parser::new(b"ensure 42 end");
    assert_eq!(
        parser.try_opt_ensure(),
        Ok(Some((
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
        )))
    );
}
