use crate::{
    parser::base::{ParseResult, Rule},
    Parser,
};

pub(crate) struct Maybe1<R>
where
    R: Rule,
{
    _r: std::marker::PhantomData<R>,
}

impl<R: Rule> Rule for Maybe1<R>
where
    R: Rule,
{
    type Output = Option<R::Output>;

    fn starts_now(parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if !R::starts_now(parser) {
            return Ok(None);
        }

        match R::parse(parser) {
            Ok(value) => Ok(Some(value)),
            Err(err) => Err(err),
        }
    }
}
