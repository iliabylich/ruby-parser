use crate::{
    builder::Builder,
    parser::base::{ParseResult, Rule},
    Node, Parser, Token, TokenKind,
};

pub(crate) struct ParenArgs;
impl Rule for ParenArgs {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}
