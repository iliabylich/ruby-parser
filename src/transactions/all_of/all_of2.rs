use crate::transactions::{
    all_of::{all_of3::AllOf3, seq_error::SeqError},
    ParseError, StepData,
};

pub(crate) struct AllOf2<A, B>
where
    StepData: From<A>,
    StepData: From<B>,
{
    pub(crate) name: &'static str,
    pub(crate) inner: Result<(A, B), SeqError>,
}
impl<A, B> AllOf2<A, B>
where
    StepData: From<A>,
    StepData: From<B>,
{
    pub(crate) fn and<C, F>(self, f: F) -> AllOf3<A, B, C>
    where
        F: FnOnce() -> Result<C, ParseError>,
        StepData: From<C>,
    {
        let Self { inner, name } = self;
        match inner {
            Ok((a, b)) => {
                match f() {
                    Ok(c) => AllOf3 {
                        name,
                        inner: Ok((a, b, c)),
                    },
                    Err(mut error) => {
                        // this the 3rd element in a sequence,
                        // so all lookahead errors must become
                        // non-lookahead
                        error.make_required();

                        AllOf3 {
                            name,
                            inner: Err(SeqError {
                                steps: vec![StepData::from(a), StepData::from(b)],
                                error,
                            }),
                        }
                    }
                }
            }
            Err(err) => AllOf3 {
                name,
                inner: Err(err),
            },
        }
    }

    pub(crate) fn unwrap(self) -> Result<(A, B), ParseError> {
        let Self { inner, name } = self;
        match inner {
            Ok((a, b)) => Ok((a, b)),
            Err(SeqError { steps, error }) => Err(ParseError::SeqError {
                name,
                steps,
                error: Box::new(error),
            }),
        }
    }
}
