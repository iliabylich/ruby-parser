use crate::{transactions::steps::Steps, Loc, TokenKind};

#[derive(Debug, PartialEq, Eq, Clone)]
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
    pub(crate) fn seq_error<S>(name: &'static str, steps: S, error: ParseError) -> Self
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
            Self::SeqError { steps, error, .. } => steps.0.is_empty() && error.is_lookahead(),
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

// weight
impl ParseError {
    #[cfg(test)]
    pub(crate) fn weight(&self) -> usize {
        match self {
            Self::TokenError { .. } => 0,
            Self::OneOfError { variants, .. } => {
                variants.iter().map(|v| v.weight()).max().unwrap_or(0)
            }
            Self::SeqError { steps, .. } => steps.0.len(),
        }
    }
}

// strip
impl ParseError {
    #[cfg(test)]
    pub(crate) fn strip_branches(&mut self, max_nest: usize) {
        match self {
            Self::TokenError { .. } => {}
            Self::OneOfError { variants, .. } => {
                if max_nest == 0 {
                    *variants = vec![];
                } else {
                    let max_weight = variants.iter().map(|v| v.weight()).max().unwrap_or(0);
                    variants.retain(|v| v.weight() == max_weight);
                    variants
                        .iter_mut()
                        .for_each(|v| v.strip_branches(max_nest - 1))
                }
            }
            Self::SeqError { error, .. } => {
                error.strip_branches(max_nest.checked_sub(1).unwrap_or(0))
            }
        }
    }
}
