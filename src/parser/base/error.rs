#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ParseError {
    pub(crate) error: (),
}

pub(crate) type ParseResult<T> = Result<T, ParseError>;
