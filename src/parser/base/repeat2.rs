use crate::{
    parser::base::{Captured, ParseResult, Rule},
    Parser,
};

pub(crate) struct Repeat2<R1, R2>
where
    R1: Rule,
    R2: Rule,
    Captured: From<R1::Output>,
    Captured: From<R2::Output>,
{
    _r1: std::marker::PhantomData<R1>,
    _r2: std::marker::PhantomData<R2>,
}

impl<R1, R2> Rule for Repeat2<R1, R2>
where
    R1: Rule,
    R2: Rule,
    Captured: From<R1::Output>,
    Captured: From<R2::Output>,
{
    type Output = (Vec<R1::Output>, Vec<R2::Output>);

    fn starts_now(parser: &mut Parser) -> bool {
        R1::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let mut v1s = vec![];
        let mut v2s = vec![];

        loop {
            if !R1::starts_now(parser) {
                break;
            }

            match R1::parse(parser) {
                Ok(v1) => v1s.push(v1),
                Err(mut err) => {
                    err.captured = Captured::from(v1s) + Captured::from(v2s) + err.captured;

                    return Err(err);
                }
            }

            if !R2::starts_now(parser) {
                break;
            }

            match R2::parse(parser) {
                Ok(v2) => v2s.push(v2),
                Err(mut err) => {
                    err.captured = Captured::from(v1s) + Captured::from(v2s) + err.captured;

                    return Err(err);
                }
            }
        }

        Ok((v1s, v2s))
    }
}
