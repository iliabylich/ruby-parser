use crate::{parser::base::Rule, Parser};

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

    fn starts_now(_parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if !R::starts_now(parser) {
            return None;
        }

        Some(R::parse(parser))
    }
}
