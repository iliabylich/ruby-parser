use crate::{lexer::Checkpoint, token::TokenKind};

#[derive(Debug)]
pub(crate) struct ParseResultChain<T> {
    checkpoint: Checkpoint,
    inner: Result<T, ParseError>,
}

impl<T> ParseResultChain<T> {
    pub(crate) fn new(checkpoint: Checkpoint) -> Self {
        Self {
            checkpoint,
            inner: Err(ParseError::empty()),
        }
    }

    pub(crate) fn or_else<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Result<T, ParseError>,
    {
        let Self { checkpoint, inner } = self;

        match inner {
            ok @ Ok(_) => {
                // we had a match, no need to run anything
                Self {
                    checkpoint,
                    inner: ok,
                }
            }
            Err(mut error) => {
                let fallback = f();
                match fallback {
                    ok @ Ok(_) => {
                        // we've got a match, return it
                        Self {
                            checkpoint,
                            inner: ok,
                        }
                    }
                    Err(mut new_error) => {
                        // record errors
                        error.expectations.append(&mut new_error.expectations);
                        // perform a rollback
                        checkpoint.restore();

                        // and return an error
                        Self {
                            checkpoint,
                            inner: Err(error),
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn into_parse_result(self) -> Result<T, ParseError> {
        self.inner
    }

    pub(crate) fn handle_error<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(ParseError) -> (),
    {
        match self.inner {
            Ok(value) => Some(value),
            Err(error) => {
                f(error);
                None
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ParseError {
    pub(crate) expectations: Vec<Expectation>,
}

impl ParseError {
    pub(crate) fn empty() -> Self {
        Self {
            expectations: vec![],
        }
    }

    pub(crate) fn expected_token(expected: TokenKind, actual: TokenKind) -> Self {
        Self {
            expectations: vec![Expectation::ExpectedToken {
                expected_token: expected,
                got_token: actual,
            }],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expectation {
    ExpectedToken {
        expected_token: TokenKind,
        got_token: TokenKind,
    },
    ExpectedRule {
        rule: &'static str,
        expected_token: TokenKind,
        got_token: TokenKind,
    },

    // For initial lookahead
    LooakaheadFailed {
        expected_token: TokenKind,
        got_tokeb: TokenKind,
    },
}

#[cfg(test)]
mod tests {
    use super::{Checkpoint, Expectation, ParseError, ParseResultChain};
    use crate::{
        state::{OwnedState, StateRef},
        token::TokenKind,
    };

    const EXPECTATION: Expectation = Expectation::ExpectedToken {
        expected_token: TokenKind::tTEST_TOKEN,
        got_token: TokenKind::kALIAS,
    };

    fn setup() -> (OwnedState, StateRef, Checkpoint, usize) {
        let mut state = OwnedState::new(b"");
        let state_ref = state.new_ref();

        let initial_pos = 42;
        state.buffer_mut().set_pos(initial_pos);
        assert_eq!(state.buffer().pos(), initial_pos);

        // create a checkpoint after altering state
        let checkpoint = Checkpoint::new(state_ref);

        (state, state_ref, checkpoint, initial_pos)
    }

    #[test]
    fn test_result_ok() {
        let (state, state_ref, checkpoint, initial_pos) = setup();

        let mut errors = ParseError::empty();
        let result = ParseResultChain::new(checkpoint)
            .or_else(|| {
                assert_eq!(state_ref.buffer().pos(), initial_pos);
                state_ref.buffer().set_pos(initial_pos + 1);
                Ok(1)
            })
            .handle_error(|errs| errors = errs);

        assert_eq!(result, Some(1));
        assert_eq!(
            errors,
            ParseError {
                expectations: vec![]
            }
        );
        // make sure we DID NOT make a rollback
        assert_eq!(state.buffer().pos(), initial_pos + 1);
    }

    #[test]
    fn test_result_err() {
        let (state, state_ref, checkpoint, initial_pos) = setup();

        let mut errors = ParseError::empty();
        let result = ParseResultChain::<i32>::new(checkpoint)
            .or_else(|| {
                assert_eq!(state_ref.buffer().pos(), initial_pos);
                state_ref.buffer().set_pos(initial_pos + 1);
                Err(ParseError {
                    expectations: vec![EXPECTATION],
                })
            })
            .handle_error(|errs| errors = errs);

        assert_eq!(result, None);
        assert_eq!(
            errors,
            ParseError {
                expectations: vec![EXPECTATION]
            }
        );
        // make sure we performed a rollback
        assert_eq!(state.buffer().pos(), initial_pos);
    }

    #[test]
    fn test_result_mixed_ok_with_errors() {
        let (state, state_ref, checkpoint, initial_pos) = setup();

        let mut errors = ParseError::empty();
        let result = ParseResultChain::new(checkpoint)
            .or_else(|| {
                assert_eq!(state_ref.buffer().pos(), initial_pos);
                state_ref.buffer().set_pos(initial_pos + 1);
                Err(ParseError {
                    expectations: vec![EXPECTATION],
                })
            })
            .or_else(|| {
                assert_eq!(state_ref.buffer().pos(), initial_pos);
                state_ref.buffer().set_pos(initial_pos + 2);
                Ok(42)
            })
            .or_else(|| {
                assert_eq!(state_ref.buffer().pos(), initial_pos);
                state_ref.buffer().set_pos(initial_pos + 3);
                Err(ParseError {
                    expectations: vec![EXPECTATION],
                })
            })
            .handle_error(|errs| errors = errs);

        assert_eq!(result, Some(42));
        assert_eq!(
            errors,
            ParseError {
                expectations: vec![]
            }
        );
        // make sure we DID NOT performe a rollback
        assert_eq!(state.buffer().pos(), initial_pos + 2);
    }

    #[test]
    fn test_result_multiple_errors() {
        let (state, state_ref, checkpoint, initial_pos) = setup();

        let mut errors = ParseError::empty();
        let result = ParseResultChain::<i32>::new(checkpoint)
            .or_else(|| {
                assert_eq!(state_ref.buffer().pos(), initial_pos);
                state_ref.buffer().set_pos(initial_pos + 1);
                Err(ParseError {
                    expectations: vec![EXPECTATION],
                })
            })
            .or_else(|| {
                assert_eq!(state_ref.buffer().pos(), initial_pos);
                state_ref.buffer().set_pos(initial_pos + 2);
                Err(ParseError {
                    expectations: vec![EXPECTATION],
                })
            })
            .handle_error(|errs| errors = errs);

        assert_eq!(result, None);
        assert_eq!(
            errors,
            ParseError {
                expectations: vec![EXPECTATION, EXPECTATION]
            }
        );
        // make sure we DID NOT performe a rollback
        assert_eq!(state.buffer().pos(), initial_pos);
    }
}
