use crate::{
    parser::{ExactToken, Maybe1, ParseResult, Rule},
    Node, Parser, Token, TokenKind,
};

pub(crate) struct OptRescue;
impl Rule for OptRescue {
    type Output = Option<Box<Node>>;

    fn starts_now(parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        Maybe1::<Rescue>::parse(parser)
    }
}

struct Rescue;
impl Rule for Rescue {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

struct ExcList;
impl Rule for ExcList {
    type Output = Vec<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}

struct ExcVar;
impl Rule for ExcVar {
    type Output = Option<(Token, Box<Node>)>;

    fn starts_now(parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        type AssocT = ExactToken<{ TokenKind::tASSOC as u8 }>;
        todo!("depends on LHS")
        // Maybe2::<AssocT, LHS>::parse(parser)
    }
}
