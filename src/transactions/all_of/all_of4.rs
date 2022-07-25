use crate::transactions::{all_of::seq_error::SeqError, ParseError, StepData};

pub(crate) struct AllOf4<A, B, C, D>
where
    StepData: From<A>,
    StepData: From<B>,
    StepData: From<C>,
    StepData: From<D>,
{
    pub(crate) name: &'static str,
    pub(crate) inner: Result<(A, B, C, D), SeqError>,
}
impl<A, B, C, D> AllOf4<A, B, C, D>
where
    StepData: From<A>,
    StepData: From<B>,
    StepData: From<C>,
    StepData: From<D>,
{
    pub(crate) fn unwrap(self) -> Result<(A, B, C, D), ParseError> {
        let Self { inner, name } = self;
        match inner {
            Ok((a, b, c, d)) => Ok((a, b, c, d)),
            Err(SeqError { steps, error }) => Err(ParseError::SeqError {
                name,
                steps,
                error: Box::new(error),
            }),
        }
    }
}
