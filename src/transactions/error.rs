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
        steps: Steps,
        error: Box<ParseError>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Steps(pub(crate) Vec<StepData>);

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
// Option<tuple> as Mixed variant
impl<A, B> From<Option<(A, B)>> for StepData
where
    StepData: From<A>,
    StepData: From<B>,
{
    fn from(data: Option<(A, B)>) -> Self {
        if let Some((a, b)) = data {
            Self::from((a, b))
        } else {
            Self::Mixed(vec![])
        }
    }
}
impl<A, B, C> From<Option<(A, B, C)>> for StepData
where
    StepData: From<A>,
    StepData: From<B>,
    StepData: From<C>,
{
    fn from(data: Option<(A, B, C)>) -> Self {
        if let Some((a, b, c)) = data {
            Self::from((a, b, c))
        } else {
            Self::Mixed(vec![])
        }
    }
}
impl From<Token> for Steps {
    fn from(token: Token) -> Self {
        Steps(vec![token.into()])
    }
}
impl From<Node> for Steps {
    fn from(node: Node) -> Self {
        Steps(vec![Box::new(node).into()])
    }
}
impl From<Box<Node>> for Steps {
    fn from(node: Box<Node>) -> Self {
        Steps(vec![node.into()])
    }
}
impl From<Vec<Node>> for Steps {
    fn from(nodes: Vec<Node>) -> Self {
        Steps(
            nodes
                .into_iter()
                .map(|node| Box::new(node).into())
                .collect(),
        )
    }
}
impl From<Vec<Token>> for Steps {
    fn from(tokens: Vec<Token>) -> Self {
        Steps(tokens.into_iter().map(|token| token.into()).collect())
    }
}
impl<A, B> From<(A, B)> for Steps
where
    StepData: From<A>,
    StepData: From<B>,
{
    fn from((lhs, rhs): (A, B)) -> Self {
        todo!()
    }
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
