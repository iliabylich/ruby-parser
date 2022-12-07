use crate::{
    parser::base::{ParseResult, Rule},
    Node, Parser,
};

pub(crate) struct Params;
impl Rule for Params {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}
