use crate::{parser::base::Rule, Parser};

pub(crate) struct Maybe3<R1, R2, R3>
where
    R1: Rule,
    R2: Rule,
    R3: Rule,
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
{
    type Output = Option<(R1::Output, R2::Output, R3::Output)>;

    fn starts_now(parser: &mut Parser) -> bool {
        R1::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if !R1::starts_now(parser) {
            return None;
        }

        let v1 = R1::parse(parser);
        let v2 = R2::parse(parser);
        let v3 = R3::parse(parser);

        Some((v1, v2, v3))
    }
}
