pub(crate) use crate::{
    parser::base::{ParseResult, Rule},
    Node, Parser,
};

pub(crate) struct MaybeCommandBlock;
impl Rule for MaybeCommandBlock {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}
