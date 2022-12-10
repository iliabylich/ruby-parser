use crate::{
    parser::base::{ExactToken, Maybe1, Maybe2, ParseResult, Rule, SeparatedBy},
    Node, Parser, TokenKind,
};

pub(crate) struct DoBlock;
impl Rule for DoBlock {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kDO)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

pub(crate) type MaybeBraceBlock = Maybe1<BraceBlock>;

pub(crate) struct Block;
impl Rule for Block {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        BraceBlock::starts_now(parser) || DoBlock::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

pub(crate) struct BraceBlock;
impl Rule for BraceBlock {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tLCURLY)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}
