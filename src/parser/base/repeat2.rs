use crate::{
    parser::base::{ParseResult, Rule},
    Parser,
};

pub(crate) struct Repeat2<R1, R2>
where
    R1: Rule,
    R2: Rule,
{
    _r1: std::marker::PhantomData<R1>,
    _r2: std::marker::PhantomData<R2>,
}

impl<R1, R2> Rule for Repeat2<R1, R2>
where
    R1: Rule,
    R2: Rule,
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

            let v1 = R1::parse(parser).unwrap();
            v1s.push(v1);

            if !R2::starts_now(parser) {
                break;
            }

            let v2 = R2::parse(parser).unwrap();
            v2s.push(v2);
        }

        Ok((v1s, v2s))
    }
}
