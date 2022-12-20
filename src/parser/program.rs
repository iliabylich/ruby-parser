use crate::{parser::base::Rule, Node, Parser};

pub(crate) struct Program;
impl Rule for Program {
    type Output = Option<Box<Node>>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        todo!()
    }
}
