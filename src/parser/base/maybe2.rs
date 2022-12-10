use crate::{
    parser::base::{Captured, ParseResult, Rule},
    Parser,
};

pub(crate) struct Maybe2<R1, R2>
where
    R1: Rule,
    R2: Rule,
    Captured: From<R1::Output>,
    Captured: From<R2::Output>,
{
    _r1: std::marker::PhantomData<R1>,
    _r2: std::marker::PhantomData<R2>,
}

impl<R1, R2> Rule for Maybe2<R1, R2>
where
    R1: Rule,
    R2: Rule,
    Captured: From<R1::Output>,
    Captured: From<R2::Output>,
{
    type Output = Option<(R1::Output, R2::Output)>;

    fn starts_now(_parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if !R1::starts_now(parser) {
            return Ok(None);
        }

        let v1 = R1::parse(parser).unwrap();
        let v2 = R2::parse(parser).unwrap();

        Ok(Some((v1, v2)))
    }
}
