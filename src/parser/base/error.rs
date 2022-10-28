use crate::parser::base::Captured;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ParseError {
    pub(crate) error: (),
    pub(crate) captured: Captured,
}

pub(crate) type ParseResult<T> = Result<T, ParseError>;
