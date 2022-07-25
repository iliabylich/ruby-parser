use crate::transactions::{
    all_of::{all_of2::AllOf2, seq_error::SeqError},
    ParseError, StepData,
};

pub(crate) struct AllOf1<A>
where
    StepData: From<A>,
{
    pub(crate) name: &'static str,
    pub(crate) inner: Result<A, SeqError>,
}
impl<A> AllOf1<A>
where
    StepData: From<A>,
{
    pub(crate) fn and<B, F>(self, f: F) -> AllOf2<A, B>
    where
        F: FnOnce() -> Result<B, ParseError>,
        StepData: From<B>,
    {
        let Self { name, inner } = self;

        match inner {
            Ok(a) => {
                match f() {
                    Ok(b) => AllOf2 {
                        name,
                        inner: Ok((a, b)),
                    },
                    Err(mut error) => {
                        // this the 2nd element in a sequence,
                        // so all lookahead errors must become
                        // non-lookahead
                        error.make_required();

                        AllOf2 {
                            name,
                            inner: Err(SeqError {
                                steps: vec![StepData::from(a)],
                                error,
                            }),
                        }
                    }
                }
            }
            Err(error) => AllOf2 {
                name,
                inner: Err(error),
            },
        }
    }
}
