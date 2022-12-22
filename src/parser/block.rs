use crate::{
    parser::base::{ExactToken, Maybe1, Maybe2, Rule, SeparatedBy},
    Node, Parser, Token, TokenKind,
};

pub(crate) type MaybeBlock = Maybe1<Block>;

pub(crate) struct Block;
impl Rule for Block {
    type Output = (Token, Box<Node>, Token);

    fn starts_now(parser: &mut Parser) -> bool {
        BraceBlock::starts_now(parser) || DoBlock::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        todo!()
    }
}

struct DoBlock;
impl Rule for DoBlock {
    type Output = (Token, Box<Node>, Token);

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kDO)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        todo!()
    }
}

pub(crate) struct BraceBlock;
impl Rule for BraceBlock {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tLCURLY)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        todo!()
    }
}
