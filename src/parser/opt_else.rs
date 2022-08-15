use crate::{
    parser::{
        macros::{all_of, one_of},
        ParseResult, Parser,
    },
    token::{Token, TokenKind},
    Node,
};

type Else = (Token, Option<Box<Node>>);

impl Parser {
    pub(crate) fn try_opt_else(&mut self) -> ParseResult<Option<Else>> {
        one_of!(
            "opt else",
            checkpoint = self.new_checkpoint(),
            parse_else(self).map(|v| Some(v)),
            Ok(None),
        )
    }
}

fn parse_else(parser: &mut Parser) -> ParseResult<Else> {
    all_of!(
        "else",
        parser.try_token(TokenKind::kELSE),
        parser.try_compstmt(),
    )
}

#[test]
fn test_opt_else() {
    use crate::{
        loc::loc, nodes::Int, parser::Parser, string_content::StringContent, token::token, Node,
    };
    let mut parser = Parser::new(b"else 42 end").debug();
    assert_eq!(
        parser.try_opt_else(),
        Ok(Some((
            token!(kELSE, loc!(0, 4)),
            Some(Box::new(Node::Int(Int {
                value: StringContent::from("42"),
                operator_l: None,
                expression_l: loc!(5, 7)
            })))
        )))
    )
}
