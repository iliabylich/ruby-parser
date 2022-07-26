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

    None,
}

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum StepData {
    Node(Box<Node>),
    MaybeNode(Option<Box<Node>>),
    Nodes(Vec<Node>),

    Token(Token),
    MaybeToken(Option<Token>),
    Tokens(Vec<Token>),

    Mixed(Vec<StepData>),
}
impl From<Box<Node>> for StepData {
    fn from(node: Box<Node>) -> Self {
        Self::Node(node)
    }
}
impl From<Option<Box<Node>>> for StepData {
    fn from(maybe_node: Option<Box<Node>>) -> Self {
        Self::MaybeNode(maybe_node)
    }
}
impl From<Vec<Node>> for StepData {
    fn from(nodes: Vec<Node>) -> Self {
        Self::Nodes(nodes)
    }
}

impl From<Token> for StepData {
    fn from(token: Token) -> Self {
        Self::Token(token)
    }
}
impl From<Option<Token>> for StepData {
    fn from(maybe_token: Option<Token>) -> Self {
        Self::MaybeToken(maybe_token)
    }
}
impl From<Vec<Token>> for StepData {
    fn from(tokens: Vec<Token>) -> Self {
        Self::Tokens(tokens)
    }
}

// tuples as Mixed variant
impl<A, B> From<(A, B)> for StepData
where
    StepData: From<A>,
    StepData: From<B>,
{
    fn from((a, b): (A, B)) -> Self {
        Self::Mixed(vec![Self::from(a), Self::from(b)])
    }
}
impl<A, B, C> From<(A, B, C)> for StepData
where
    StepData: From<A>,
    StepData: From<B>,
    StepData: From<C>,
{
    fn from((a, b, c): (A, B, C)) -> Self {
        Self::Mixed(vec![Self::from(a), Self::from(b), Self::from(c)])
    }
}
impl<A, B, C, D> From<(A, B, C, D)> for StepData
where
    StepData: From<A>,
    StepData: From<B>,
    StepData: From<C>,
    StepData: From<D>,
{
    fn from((a, b, c, d): (A, B, C, D)) -> Self {
        Self::Mixed(vec![
            Self::from(a),
            Self::from(b),
            Self::from(c),
            Self::from(d),
        ])
    }
}

// is_lookahead
impl ParseError {
    pub(crate) fn is_lookahead(&self) -> bool {
        match self {
            Self::TokenError { lookahead, .. } => *lookahead,
            Self::OneOfError { variants, .. } => variants.iter().all(|v| v.is_lookahead()),
            Self::SeqError { steps, .. } => steps.is_empty(),
            Self::None => false,
        }
    }
}

// strip_lookaheads
impl ParseError {
    pub(crate) fn strip_lookaheads(self) -> Self {
        match self {
            Self::OneOfError { mut variants, name } => {
                variants.retain(|v| !v.is_lookahead());
                if variants.is_empty() {
                    return Self::None;
                }
                Self::OneOfError { name, variants }
            }

            err @ Self::None | err @ Self::TokenError { .. } => err,

            Self::SeqError { error, name, steps } => {
                let error = error.strip_lookaheads();
                if error == Self::None {
                    return Self::None;
                }
                let error = Box::new(error);
                Self::SeqError { name, steps, error }
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
            ParseError::None => {}
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
            Self::SeqError { steps, .. } => 10 * steps.len() + 1,
            Self::None => 0,
        }
    }
}
