use crate::{
    parser::{
        macros::{all_of, one_of},
        ParseResult, Parser,
    },
    token::{Token, TokenKind},
    Node,
};

type Ensure = (Token, Option<Box<Node>>);

impl Parser {
    pub(crate) fn try_opt_ensure(&mut self) -> ParseResult<Option<Ensure>> {
        one_of!(
            "opt ensure",
            checkpoint = self.new_checkpoint(),
            parse_ensure(self).map(|v| Some(v)),
            Ok(None),
        )
    }
}

fn parse_ensure(parser: &mut Parser) -> ParseResult<Ensure> {
    all_of!(
        "ensure",
        parser.try_token(TokenKind::kENSURE),
        parser.try_compstmt(),
    )
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
