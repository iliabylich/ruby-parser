use crate::{Loc, Node, Token, TokenKind};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ParseError {
    pub(crate) name: &'static str,
    pub(crate) details: ParseErrorDetails,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ParseErrorDetails {
    Multi { variants: Vec<ParseError> },
    Seq { steps: Vec<StepError> },
    Single { inner: Expectation },
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct StepError {
    pub(crate) name: &'static str,
    pub(crate) expectation: Expectation,
    pub(crate) inner: ParseError,
    pub(crate) data: StepData,
}
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum StepData {
    Token(Token),
    Node(Box<Node>),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Expectation {
    pub(crate) lookahead: bool,
    pub(crate) expected: TokenKind,
    pub(crate) got: TokenKind,
    pub(crate) loc: Loc,
}

// is_lookahead
impl ParseError {
    pub(crate) fn is_lookahead(&self) -> bool {
        self.details.is_lookahead()
    }
}
impl ParseErrorDetails {
    pub(crate) fn is_lookahead(&self) -> bool {
        match self {
            Self::Multi { variants } => variants.iter().all(|v| v.is_lookahead()),
            Self::Seq { steps } => steps.len() == 1 && steps.first().unwrap().is_lookahead(),
            Self::Single { inner } => inner.is_lookahead(),
        }
    }
}
impl StepError {
    pub(crate) fn is_lookahead(&self) -> bool {
        self.inner.is_lookahead()
    }
}
impl Expectation {
    pub(crate) fn is_lookahead(&self) -> bool {
        self.lookahead
    }
}

// strip_lookahead_errors
impl ParseError {
    pub(crate) fn strip_lookahead_errors(mut self) -> Option<Self> {
        if self.is_lookahead() {
            return None;
        }

        match &mut self.details {
            ParseErrorDetails::Multi { variants } => {
                variants.retain(|v| v.is_lookahead());
            }

            // The following variants are kept
            // if they are not lookaheads (checked above)
            ParseErrorDetails::Seq { .. } => {}
            ParseErrorDetails::Single { .. } => {}
        }

        Some(self)
    }
}

// is_empty
impl ParseError {
    pub(crate) fn is_empty(&self) -> bool {
        match &self.details {
            ParseErrorDetails::Multi { variants } => !variants.is_empty(),
            ParseErrorDetails::Seq { steps } => !steps.is_empty(),
            ParseErrorDetails::Single { .. } => false,
        }
    }
}

// empty
impl ParseError {
    pub(crate) fn empty() -> Self {
        Self {
            name: "(empty)",
            details: ParseErrorDetails::Multi { variants: vec![] },
        }
    }
}
