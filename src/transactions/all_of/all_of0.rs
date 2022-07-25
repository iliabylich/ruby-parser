use crate::transactions::{
    all_of::{all_of1::AllOf1, seq_error::SeqError},
    ParseError, StepData,
};

pub(crate) struct AllOf0 {
    name: &'static str,
}
impl AllOf0 {
    pub(crate) fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub(crate) fn and<A, F>(self, f: F) -> AllOf1<A>
    where
        F: FnOnce() -> Result<A, ParseError>,
        StepData: From<A>,
    {
        let Self { name } = self;
        match f() {
            Ok(a) => AllOf1 { name, inner: Ok(a) },
            Err(error) => AllOf1 {
                name,
                inner: Err(SeqError {
                    steps: vec![],
                    error,
                }),
            },
        }
    }
}
