use crate::{lexer::Checkpoint, token::TokenKind};

pub(crate) struct ParseResult<T> {
    checkpoint: Checkpoint,
    inner: ParseResultInner<T>,
}

impl<T> ParseResult<T> {
    pub(crate) fn new(checkpoint: Checkpoint) -> Self {
        Self {
            checkpoint,
            inner: ParseResultInner::Error {
                expectations: vec![],
            },
        }
    }

    pub(crate) fn or_else<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Result<T, Expectation>,
    {
        let Self { checkpoint, inner } = self;
        match inner.or_else(f) {
            ok @ ParseResultInner::Ok(_) => {
                // match, no need to perform a rollback
                Self {
                    checkpoint,
                    inner: ok,
                }
            }
            err @ ParseResultInner::Error { .. } => {
                // no match, rollback
                panic!("rollback");
                Self {
                    checkpoint,
                    inner: err,
                }
            }
        }
    }
}

enum ParseResultInner<T> {
    Ok(T),
    Error { expectations: Vec<Expectation> },
}

pub(crate) enum Expectation {
    SingleToken(TokenKind),
    TokenAfterTokens {
        got: Vec<TokenKind>,
        expected: TokenKind,
    },
}

impl<T> ParseResultInner<T> {
    fn or_else<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Result<T, Expectation>,
    {
        match self {
            ok @ Self::Ok(_) => ok,
            Self::Error { mut expectations } => {
                let fallback = f();
                match fallback {
                    Ok(value) => Self::Ok(value),
                    Err(expectation) => {
                        expectations.push(expectation);
                        Self::Error { expectations }
                    }
                }
            }
        }
    }

    fn handle_error<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(Vec<Expectation>) -> (),
    {
        match self {
            Self::Ok(value) => Some(value),
            Self::Error { expectations } => {
                f(expectations);
                None
            }
        }
    }
}
