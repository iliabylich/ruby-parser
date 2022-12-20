use crate::{
    parser::base::{Rule, Unbox},
    Parser,
};

pub(crate) struct Repeat1<R>
where
    R: Rule,
    <R as Rule>::Output: Unbox,
{
    _r: std::marker::PhantomData<R>,
}

impl<R> Rule for Repeat1<R>
where
    R: Rule,
    <R as Rule>::Output: Unbox,
{
    type Output = Vec<<R::Output as Unbox>::Output>;

    fn starts_now(_parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let mut values = vec![];

        loop {
            if !R::starts_now(parser) {
                break;
            }

            let value = R::parse(parser).unbox();
            values.push(value);
        }
        values
    }
}
