use crate::transactions::{steps::Steps, ParseError};

pub(crate) struct SeqError {
    pub(crate) steps: Steps,
    pub(crate) error: ParseError,
}
