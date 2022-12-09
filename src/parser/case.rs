use crate::{
    builder::Builder,
    parser::base::{ParseResult, Rule},
    token::{Token, TokenKind},
    Node, Parser,
};

pub(crate) struct Case;
impl Rule for Case {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kCASE)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}
