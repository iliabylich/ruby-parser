use crate::{
    parser::base::{ParseResult, Rule, Unbox},
    Parser,
};

pub(crate) struct AtLeastOnce<R>
where
    R: Rule,
    R::Output: Unbox,
{
    _r: std::marker::PhantomData<R>,
}

impl<R> Rule for AtLeastOnce<R>
where
    R: Rule,
    R::Output: Unbox,
{
    type Output = Vec<<R::Output as Unbox>::Output>;

    fn starts_now(parser: &mut Parser) -> bool {
        R::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let mut values = vec![];

        let value = R::parse(parser).unwrap().unbox();
        values.push(value);

        loop {
            if !R::starts_now(parser) {
                break;
            }

            let value = R::parse(parser).unwrap().unbox();
            values.push(value);
        }

        Ok(values)
    }
}
