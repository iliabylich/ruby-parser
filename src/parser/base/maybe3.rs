use crate::{
    parser::base::{Captured, ParseResult, Rule},
    Parser,
};

pub(crate) struct Maybe3<R1, R2, R3>
where
    R1: Rule,
    R2: Rule,
    R3: Rule,
    Captured: From<R1::Output>,
    Captured: From<R2::Output>,
    Captured: From<R3::Output>,
{
    _r1: std::marker::PhantomData<R1>,
    _r2: std::marker::PhantomData<R2>,
    _r3: std::marker::PhantomData<R3>,
}

impl<R1, R2, R3> Rule for Maybe3<R1, R2, R3>
where
    R1: Rule,
    R2: Rule,
    R3: Rule,
    Captured: From<R1::Output>,
    Captured: From<R2::Output>,
    Captured: From<R3::Output>,
{
    type Output = Option<(R1::Output, R2::Output, R3::Output)>;

    fn starts_now(parser: &mut Parser) -> bool {
        R1::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if !R1::starts_now(parser) {
            return Ok(None);
        }

        let v1 = match R1::parse(parser) {
            Ok(v1) => v1,
            Err(err) => return Err(err),
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

        if !R3::starts_now(parser) {
            return Ok(None);
        }

        let v3 = match R3::parse(parser) {
            Ok(v3) => v3,
            Err(mut err) => {
                err.captured = Captured::from(v1) + Captured::from(v2) + err.captured;
                return Err(err);
            }
        };

        Ok(Some((v1, v2, v3)))
    }
}
