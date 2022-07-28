use crate::transactions::error::ParseError;

pub(crate) type ParseResult<T> = Result<T, ParseError>;
