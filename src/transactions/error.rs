use crate::{Loc, Node, Token, TokenKind};

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
        steps: Vec<StepData>,
        error: Box<ParseError>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum StepData {
    Token(Token),
    Node(Box<Node>),
    Nodes(Vec<Node>),
    None,
}
impl From<Token> for StepData {
    fn from(token: Token) -> Self {
        Self::Token(token)
    }
}
impl From<Box<Node>> for StepData {
    fn from(node: Box<Node>) -> Self {
        Self::Node(node)
    }
}
impl From<Vec<Node>> for StepData {
    fn from(nodes: Vec<Node>) -> Self {
        Self::Nodes(nodes)
    }
}
impl From<(Box<Node>, Box<Node>)> for StepData {
    fn from((a, b): (Box<Node>, Box<Node>)) -> Self {
        Self::Nodes(vec![*a, *b])
    }
}

// is_lookahead
impl ParseError {
    pub(crate) fn is_lookahead(&self) -> bool {
        match self {
            Self::TokenError { lookahead, .. } => *lookahead,
            Self::OneOfError { variants, .. } => variants.iter().all(|v| v.is_lookahead()),
            Self::SeqError { steps, .. } => steps.is_empty(),
        }
    }
}

// strip_lookahead_errors
impl ParseError {
    pub(crate) fn strip_lookahead_errors(mut self) -> Option<Self> {
        if self.is_lookahead() {
            return None;
        }

        match &mut self {
            Self::OneOfError { variants, .. } => {
                variants.retain(|v| v.is_lookahead());
            }

            // The following variants are kept
            // if they are not lookaheads (checked above)
            Self::TokenError { .. } => {}
            Self::SeqError { .. } => {}
        }

        Some(self)
    }
}

// empty
impl ParseError {
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

    pub(crate) fn into_required(mut self) -> Self {
        self.make_required();
        self
    }
}
