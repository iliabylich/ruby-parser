use crate::{
    parser::{ParseResult, Rule},
    Node, Parser,
};

pub(crate) struct Lambda;
impl Rule for Lambda {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}