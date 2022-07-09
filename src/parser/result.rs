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
                checkpoint.restore();
                Self {
                    checkpoint,
                    inner: err,
                }
            }
        }
    }

    fn handle_error<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(Vec<Expectation>) -> (),
    {
        self.inner.handle_error(f)
    }
}

enum ParseResultInner<T> {
    Ok(T),
    Error { expectations: Vec<Expectation> },
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expectation {
    ExpectedTokenToStartRule {
        expected: TokenKind,
        rule: &'static str,
    },
    ExpectedTokenAfterTokensToCompleteRule {
        expected: TokenKind,
        got: Vec<TokenKind>,
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

#[cfg(test)]
mod tests {
    use super::{Checkpoint, Expectation, ParseResult};
    use crate::{
        state::{OwnedState, StateRef},
        token::TokenKind,
    };

    const EXPECTATION: Expectation = Expectation::ExpectedTokenToStartRule {
        expected: TokenKind::tTEST_TOKEN,
        rule: "test",
    };

    fn setup() -> (OwnedState, StateRef, Checkpoint, usize) {
        let mut state = OwnedState::new(b"");
        let state_ref = state.new_ref();

        let initial_pos = 42;
        state.buffer_mut().set_pos(initial_pos);
        assert_eq!(state.buffer().pos(), initial_pos);

        // create a checkpoint after altering state
        let checkpoint = Checkpoint::new(state_ref);
        dbg!(&checkpoint);

        (state, state_ref, checkpoint, initial_pos)
    }

    #[test]
    fn test_result_ok() {
        let (state, state_ref, checkpoint, initial_pos) = setup();

        let mut errors = vec![];
        let result = ParseResult::new(checkpoint)
            .or_else(|| {
                assert_eq!(state_ref.buffer().pos(), initial_pos);
                state_ref.buffer().set_pos(initial_pos + 1);
                Ok(1)
            })
            .handle_error(|errs| errors = errs);

        assert_eq!(result, Some(1));
        assert_eq!(errors, vec![]);
        // make sure we DID NOT make a rollback
        assert_eq!(state.buffer().pos(), initial_pos + 1);
    }

    #[test]
    fn test_result_err() {
        let (state, state_ref, checkpoint, initial_pos) = setup();

        let mut errors = vec![];
        let result = ParseResult::<i32>::new(checkpoint)
            .or_else(|| {
                assert_eq!(state_ref.buffer().pos(), initial_pos);
                state_ref.buffer().set_pos(initial_pos + 1);
                Err(EXPECTATION)
            })
            .handle_error(|errs| errors = errs);

        assert_eq!(result, None);
        assert_eq!(errors, vec![EXPECTATION]);
        // make sure we performed a rollback
        assert_eq!(state.buffer().pos(), initial_pos);
    }

    #[test]
    fn test_result_mixed_ok_with_errors() {
        let (state, state_ref, checkpoint, initial_pos) = setup();

        let mut errors = vec![];
        let result = ParseResult::new(checkpoint)
            .or_else(|| {
                assert_eq!(state_ref.buffer().pos(), initial_pos);
                state_ref.buffer().set_pos(initial_pos + 1);
                Err(EXPECTATION)
            })
            .or_else(|| {
                assert_eq!(state_ref.buffer().pos(), initial_pos);
                state_ref.buffer().set_pos(initial_pos + 2);
                Ok(42)
            })
            .or_else(|| {
                assert_eq!(state_ref.buffer().pos(), initial_pos);
                state_ref.buffer().set_pos(initial_pos + 3);
                Err(EXPECTATION)
            })
            .handle_error(|errs| errors = errs);

        assert_eq!(result, Some(42));
        assert_eq!(errors, vec![]);
        // make sure we DID NOT performe a rollback
        assert_eq!(state.buffer().pos(), initial_pos + 2);
    }

    #[test]
    fn test_result_multiple_errors() {
        let (state, state_ref, checkpoint, initial_pos) = setup();

        let mut errors = vec![];
        let result = ParseResult::<i32>::new(checkpoint)
            .or_else(|| {
                assert_eq!(state_ref.buffer().pos(), initial_pos);
                state_ref.buffer().set_pos(initial_pos + 1);
                Err(EXPECTATION)
            })
            .or_else(|| {
                assert_eq!(state_ref.buffer().pos(), initial_pos);
                state_ref.buffer().set_pos(initial_pos + 2);
                Err(EXPECTATION)
            })
            .handle_error(|errs| errors = errs);

        assert_eq!(result, None);
        assert_eq!(errors, vec![EXPECTATION, EXPECTATION]);
        // make sure we DID NOT performe a rollback
        assert_eq!(state.buffer().pos(), initial_pos);
    }
}
