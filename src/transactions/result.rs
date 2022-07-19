use crate::transactions::error::ParseError;

pub(crate) type ParseResult<T> = Result<T, ParseError>;

pub(crate) trait ParseResultApi<T> {
    fn ignore_lookahead_errors(self) -> Result<Option<T>, ParseError>;
}

impl<T> ParseResultApi<T> for ParseResult<T> {
    fn ignore_lookahead_errors(self) -> ParseResult<Option<T>> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(error) => match error.strip_lookahead_errors() {
                Some(err) => Err(err),
                None => Ok(None),
            },
        }
    }
}
