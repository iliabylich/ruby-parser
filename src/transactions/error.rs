use crate::{transactions::steps::Steps, Loc, TokenKind};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ParseError {
    TokenError {
        lookahead: bool,
        expected: TokenKind,
        got: TokenKind,
        loc: Loc,
    },

    OneOfError {
        name: &'static str,
        variants: Vec<ParseError>,
    },

    SeqError {
        name: &'static str,
        steps: Steps,
        error: Box<ParseError>,
    },
}

impl ParseError {
    pub(crate) fn seq_error<T, S>(name: &'static str, steps: S, error: ParseError) -> Self
    where
        Steps: From<S>,
    {
        let steps = Steps::from(steps);
        Self::SeqError {
            name,
            steps,
            error: Box::new(error),
        }
    }
}

// is_lookahead
impl ParseError {
    pub(crate) fn is_lookahead(&self) -> bool {
        match self {
            Self::TokenError { lookahead, .. } => *lookahead,
            Self::OneOfError { variants, .. } => variants.iter().all(|v| v.is_lookahead()),
            Self::SeqError { steps, .. } => steps.0.is_empty(),
        }
    }
}

// strip_lookaheads
impl ParseError {
    pub(crate) fn strip_lookaheads(self) -> Option<Self> {
        match self {
            Self::OneOfError { mut variants, name } => {
                variants.retain(|v| !v.is_lookahead());
                if variants.is_empty() {
                    return None;
                }
                Some(Self::OneOfError { name, variants })
            }

            err @ Self::TokenError { .. } => {
                if err.is_lookahead() {
                    None
                } else {
                    Some(err)
                }
            }

            Self::SeqError { error, name, steps } => {
                let error = error.strip_lookaheads()?;
                let error = Box::new(error);
                Some(Self::SeqError { name, steps, error })
            }
        }
    }
}

// empty
impl ParseError {
    #[cfg(test)]
    pub(crate) fn empty() -> Self {
        Self::OneOfError {
            name: "(empty)",
            variants: vec![],
        }
    }
}

// into_required
impl ParseError {
    pub(crate) fn make_required(&mut self) {
        match self {
            ParseError::TokenError { lookahead, .. } => {
                *lookahead = false;
            }
            ParseError::OneOfError { variants, .. } => {
                variants.iter_mut().for_each(|e| e.make_required());
            }
            ParseError::SeqError { error, .. } => {
                error.make_required();
            }
        }
    }

    // pub(crate) fn into_required(mut self) -> Self {
    //     self.make_required();
    //     self
    // }
}

// weight
impl ParseError {
    pub(crate) fn weight(&self) -> usize {
        match self {
            Self::TokenError { .. } => 1,
            Self::OneOfError { variants, .. } => {
                variants.iter().map(|v| v.weight()).max().unwrap_or(0)
            }
            Self::SeqError { steps, .. } => 10 * steps.0.len() + 1,
        }
    }
}
