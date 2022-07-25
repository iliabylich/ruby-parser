use crate::transactions::{ParseError, StepData};

pub(crate) struct SeqError {
    pub(crate) steps: Vec<StepData>,
    pub(crate) error: ParseError,
}
