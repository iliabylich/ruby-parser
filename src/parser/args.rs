use crate::{
    builder::Builder,
    parser::{
        base::{Maybe1, Rule},
        Command, ParseResult,
    },
    Node, Parser, Token, TokenKind,
};

pub(crate) struct ParenArgs;
impl Rule for ParenArgs {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        Command::starts_now(parser) || Arglist::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

pub(crate) type OptParenArgs = Maybe1<ParenArgs>;

pub(crate) struct Args;
impl Rule for Args {
    type Output = Vec<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

struct Arglist;
impl Rule for Arglist {
    type Output = Vec<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}
