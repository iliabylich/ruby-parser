use crate::{
    parser::base::{Captured, ParseResult, Rule, Unbox},
    Parser,
};

pub(crate) struct AtLeastOnce<R>
where
    R: Rule,
    R::Output: Unbox,
    Captured: From<<R::Output as Unbox>::Output>,
{
    _r: std::marker::PhantomData<R>,
}

impl<R> Rule for AtLeastOnce<R>
where
    R: Rule,
    R::Output: Unbox,
    Captured: From<<R::Output as Unbox>::Output>,
{
    type Output = Vec<<R::Output as Unbox>::Output>;

    fn starts_now(parser: &mut Parser) -> bool {
        R::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let mut values = vec![];

        match R::parse(parser) {
            Ok(value) => values.push(value.unbox()),
            Err(err) => return Err(err),
        }

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
