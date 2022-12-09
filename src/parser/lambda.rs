use crate::{
    parser::base::{ParseResult, Rule},
    Node, Parser, TokenKind,
};

pub(crate) struct Lambda;
impl Rule for Lambda {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tLAMBDA)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}
