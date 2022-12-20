use crate::Parser;

pub(crate) trait Rule<const N: usize = 0> {
    type Output;

    fn starts_now(parser: &mut Parser) -> bool;
    fn parse(parser: &mut Parser) -> Self::Output;
}
