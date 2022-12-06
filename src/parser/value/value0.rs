use crate::{
    parser::base::{ParseResult, Rule},
    Node, Parser,
};

pub(crate) struct Value0;
impl Rule for Value0 {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}
