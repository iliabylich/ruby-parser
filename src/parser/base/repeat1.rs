use crate::{
    parser::base::{Captured, ParseResult, Rule},
    Parser,
};

pub(crate) struct Repeat1<R>
where
    R: Rule,
    Captured: From<R::Output>,
{
    _r: std::marker::PhantomData<R>,
}

impl<R> Rule for Repeat1<R>
where
    R: Rule,
    Captured: From<R::Output>,
{
    type Output = Vec<R::Output>;

    fn starts_now(parser: &mut Parser) -> bool {
        R::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let mut values = vec![];
        loop {
            if !R::starts_now(parser) {
                break;
            }

            match R::parse(parser) {
                Ok(value) => values.push(value),
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
