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

    fn starts_now(parser: &mut Parser) -> bool {
        R1::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if !R1::starts_now(parser) {
            return Ok(None);
        }

        let v1 = match R1::parse(parser) {
            Ok(v1) => v1,
            Err(error) => return Err(error),
        };

        if !R2::starts_now(parser) {
            return Ok(None);
        }

        let v2 = match R2::parse(parser) {
            Ok(v2) => v2,
            Err(mut err) => {
                err.captured = Captured::from(v1) + err.captured;
                return Err(err);
            }
        };

        Ok(Some((v1, v2)))
    }
}
