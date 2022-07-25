use crate::transactions::error::ParseError;

pub(crate) type ParseResult<T> = Result<T, ParseError>;

pub(crate) trait ParseResultApi<T> {
    fn ignore_lookaheads(self) -> Result<Option<T>, ParseError>;
}

impl<T> ParseResultApi<T> for ParseResult<T> {
    fn ignore_lookaheads(self) -> ParseResult<Option<T>> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(error) => match error.strip_lookaheads() {
                ParseError::None => Ok(None),
                err @ _ => Err(err),
            },
        }
    }
}
