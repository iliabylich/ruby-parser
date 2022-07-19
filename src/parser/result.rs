use crate::{builder::Constructor, lexer::Checkpoint, parser::Parser, token::TokenKind};

#[derive(Debug)]
pub(crate) struct OneOf<T> {
    checkpoint: Checkpoint,
    name: &'static str,
    inner: Result<T, ParseError>,
}

impl<T> OneOf<T>
where
    T: std::fmt::Debug,
{
    pub(crate) fn new(name: &'static str, checkpoint: Checkpoint) -> Self {
        Self {
            checkpoint,
            name,
            inner: Err(ParseError::empty()),
        }
    }

    pub(crate) fn or_else<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Result<T, ParseError>,
    {
        let Self {
            checkpoint,
            name,
            inner,
        } = self;

        match inner {
            ok @ Ok(_) => {
                // we had a match, no need to run anything
                Self {
                    checkpoint,
                    name,
                    inner: ok,
                }
            }
            Err(mut error) => {
                let fallback = f();
                match dbg!(fallback) {
                    ok @ Ok(_) => {
                        // we've got a match, return it
                        Self {
                            checkpoint,
                            name,
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
                            name,
                            inner: Err(error),
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn done(self) -> Result<T, ParseError> {
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

    pub(crate) fn required(mut self) -> Self {
        if let Err(error) = &mut self.inner {
            error.expectations = std::mem::take(&mut error.expectations)
                .into_iter()
                .map(|exp| exp.into_required())
                .collect();
        }
        self
    }

    pub(crate) fn strip(mut self) -> Self {
        if let Err(error) = &mut self.inner {
            debug_assert!(
                !error.expectations.is_empty(),
                "can't strip empty sequence of expectations"
            );

            let got_token = error
                .expectations
                .iter()
                .map(|exp| exp.got_token())
                .next()
                .unwrap();

            if error.expectations.iter().all(|exp| exp.is_lookahead()) {
                error.expectations = vec![Expectation::RuleLookaheadFailed {
                    rule: self.name,
                    got_token,
                }]
            } else {
                error.expectations = vec![Expectation::ExpectedRule {
                    rule: self.name,
                    got_token,
                }]
            }
        }

        self
    }
}

impl<C: Constructor> Parser<C> {
    pub(crate) fn one_of<T: std::fmt::Debug>(&self, name: &'static str) -> OneOf<T> {
        OneOf::new(name, self.new_checkpoint())
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

    pub(crate) fn expected_rule(rule: &'static str, actual: TokenKind) -> Self {
        Self {
            expectations: vec![Expectation::ExpectedRule {
                rule,
                got_token: actual,
            }],
        }
    }

    pub(crate) fn lookahead_failed(expected: TokenKind, actual: TokenKind) -> Self {
        Self {
            expectations: vec![Expectation::TokenLooakaheadFailed {
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
        got_token: TokenKind,
    },

    TokenLooakaheadFailed {
        expected_token: TokenKind,
        got_token: TokenKind,
    },
    RuleLookaheadFailed {
        rule: &'static str,
        got_token: TokenKind,
    },
}

enum ExpectationKind {
    LookaheadFailed,
    Expectation,
}

impl Expectation {
    fn is_lookahead(&self) -> bool {
        match self {
            Self::ExpectedToken { .. } | Self::ExpectedRule { .. } => false,
            Self::TokenLooakaheadFailed { .. } | Self::RuleLookaheadFailed { .. } => true,
        }
    }

    fn into_required(self) -> Self {
        match self {
            Self::ExpectedToken { .. } | Self::ExpectedRule { .. } => self,
            Self::TokenLooakaheadFailed {
                expected_token,
                got_token,
            } => Self::ExpectedToken {
                expected_token,
                got_token,
            },
            Self::RuleLookaheadFailed { rule, got_token } => Self::ExpectedRule { rule, got_token },
        }
    }

    fn got_token(&self) -> TokenKind {
        match self {
            Self::ExpectedToken { got_token, .. }
            | Self::ExpectedRule { got_token, .. }
            | Self::TokenLooakaheadFailed { got_token, .. }
            | Self::RuleLookaheadFailed { got_token, .. } => *got_token,
        }
    }

    fn kind(&self) -> ExpectationKind {
        match self {
            Self::ExpectedToken { .. } | Self::ExpectedRule { .. } => ExpectationKind::Expectation,
            Self::TokenLooakaheadFailed { .. } | Self::RuleLookaheadFailed { .. } => {
                ExpectationKind::LookaheadFailed
            }
        }
    }
}

pub(crate) trait ParserResultApi<T> {
    fn ignore_lookahead_errors(self) -> Result<Option<T>, ParseError>;
}

impl<T> ParserResultApi<T> for Result<T, ParseError> {
    fn ignore_lookahead_errors(self) -> Result<Option<T>, ParseError> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(mut error) => {
                error.expectations = error
                    .expectations
                    .into_iter()
                    .filter(|exp| exp.is_lookahead())
                    .collect();
                if error.expectations.is_empty() {
                    Ok(None)
                } else {
                    Err(error)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Checkpoint, Expectation, OneOf, ParseError};
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
        let result = OneOf::new("foo", checkpoint)
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
        let result = OneOf::<i32>::new("foo", checkpoint)
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
        let result = OneOf::new("foo", checkpoint)
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
        let result = OneOf::<i32>::new("foo", checkpoint)
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
