use crate::{parser::base::Rule, Parser};

pub(crate) struct Maybe2<R1, R2>
where
    R1: Rule,
    R2: Rule,
{
    _r1: std::marker::PhantomData<R1>,
    _r2: std::marker::PhantomData<R2>,
}

impl<R1, R2> Rule for Maybe2<R1, R2>
where
    R1: Rule,
    R2: Rule,
{
    type Output = Option<(R1::Output, R2::Output)>;

    fn starts_now(_parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if !R1::starts_now(parser) {
            return None;
        }

        let v1 = R1::parse(parser);
        let v2 = R2::parse(parser);

        Some((v1, v2))
    }
}
