use crate::{
    parser::base::{Captured, ParseResult, Rule, Unbox},
    Parser,
};

pub(crate) struct Repeat1<R>
where
    R: Rule,
    <R as Rule>::Output: Unbox,
    Captured: From<<R::Output as Unbox>::Output>,
{
    _r: std::marker::PhantomData<R>,
}

impl<R> Rule for Repeat1<R>
where
    R: Rule,
    <R as Rule>::Output: Unbox,
    Captured: From<<R::Output as Unbox>::Output>,
{
    type Output = Vec<<R::Output as Unbox>::Output>;

    fn starts_now(_parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let mut values = vec![];
        loop {
            if !R::starts_now(parser) {
                break;
            }

            match R::parse(parser) {
                Ok(value) => values.push(value.unbox()),
                Err(mut err) => {
                    let values: Captured = values.into();
                    err.captured = values + err.captured;

                    return Err(err);
                }
            }
        }
        Ok(values)
    }
}
