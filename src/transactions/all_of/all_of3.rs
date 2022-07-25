use crate::transactions::{all_of::seq_error::SeqError, ParseError, StepData};

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
