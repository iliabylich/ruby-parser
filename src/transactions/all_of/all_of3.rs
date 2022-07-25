use crate::transactions::{
    all_of::{all_of4::AllOf4, seq_error::SeqError},
    ParseError, StepData,
};

pub(crate) struct AllOf3<A, B, C>
where
    StepData: From<A>,
    StepData: From<B>,
    StepData: From<C>,
{
    pub(crate) name: &'static str,
    pub(crate) inner: Result<(A, B, C), SeqError>,
}
impl<A, B, C> AllOf3<A, B, C>
where
    StepData: From<A>,
    StepData: From<B>,
    StepData: From<C>,
{
    pub(crate) fn and<D, F>(self, f: F) -> AllOf4<A, B, C, D>
    where
        F: FnOnce() -> Result<D, ParseError>,
        StepData: From<D>,
    {
        let Self { inner, name } = self;
        match inner {
            Ok((a, b, c)) => {
                match f() {
                    Ok(d) => AllOf4 {
                        name,
                        inner: Ok((a, b, c, d)),
                    },
                    Err(mut error) => {
                        // this the 3rd element in a sequence,
                        // so all lookahead errors must become
                        // non-lookahead
                        error.make_required();

                        AllOf4 {
                            name,
                            inner: Err(SeqError {
                                steps: vec![
                                    StepData::from(a),
                                    StepData::from(b),
                                    StepData::from(c),
                                ],
                                error,
                            }),
                        }
                    }
                }
            }
            Err(err) => AllOf4 {
                name,
                inner: Err(err),
            },
        }
    }

    pub(crate) fn unwrap(self) -> Result<(A, B, C), ParseError> {
        let Self { inner, name } = self;
        match inner {
            Ok((a, b, c)) => Ok((a, b, c)),
            Err(SeqError { steps, error }) => Err(ParseError::SeqError {
                name,
                steps,
                error: Box::new(error),
            }),
        }
    }
}
