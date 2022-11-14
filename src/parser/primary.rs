use crate::{
    parser::{Array, Hash, Literal, ParseResult, Rule, VarRefT},
    Node, Parser,
};

pub(crate) struct Primary;
impl Rule for Primary {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        PrimaryHead::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

struct PrimaryHead;
impl Rule for PrimaryHead {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        Literal::starts_now(parser) || Array::starts_now(parser) || Hash::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}
