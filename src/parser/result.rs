use crate::{lexer::Checkpoint, token::TokenKind};

pub(crate) struct ParseResult<T, E> {
    checkpoint: Checkpoint,
    inner: ParseResultInner<T, E>,
}

impl<T, E> ParseResult<T, E> {
    pub(crate) fn new(checkpoint: Checkpoint) -> Self {
        todo!()
    }
}

enum ParseResultInner<T> {
    Output(T),
    ExpectationsFailed { expectations: Vec<TokenKind> },
}

impl<T, E> ParseResultInner<T, E>
where
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
    // pub(crate) fn ok()
    pub(crate) fn or_else<F>(self, f: F) -> Self
    where
        F: FnOnce(E) -> Self,
    {
        match self {
            ok @ Self::Ok(_) => ok,
            Self::Err(err) => f(err),
        }
    }
}
