pub(crate) use crate::{parser::base::Rule, Node, Parser};

pub(crate) struct MaybeCommandBlock;
impl Rule for MaybeCommandBlock {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        todo!()
    }
}
