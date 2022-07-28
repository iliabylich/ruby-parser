use crate::{transactions::step_data::StepData, Node, Token};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Steps(pub(crate) Vec<StepData>);

impl Steps {
    pub(crate) fn empty() -> Self {
        Self(vec![])
    }
}
impl std::ops::AddAssign for Steps {
    fn add_assign(&mut self, mut rhs: Self) {
        self.0.append(&mut rhs.0)
    }
}
impl std::ops::Add for Steps {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
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
impl<T> From<Option<T>> for Steps
where
    Steps: From<T>,
{
    fn from(maybe: Option<T>) -> Self {
        match maybe {
            Some(value) => Steps::from(value),
            None => Steps::empty(),
        }
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
    Steps: From<A>,
    Steps: From<B>,
{
    fn from((a, b): (A, B)) -> Self {
        Steps::from(a) + Steps::from(b)
    }
}
impl<A, B, C> From<(A, B, C)> for Steps
where
    Steps: From<A>,
    Steps: From<B>,
    Steps: From<C>,
{
    fn from((a, b, c): (A, B, C)) -> Self {
        Steps::from(a) + Steps::from(b) + Steps::from(c)
    }
}
impl<A, B, C, D> From<(A, B, C, D)> for Steps
where
    Steps: From<A>,
    Steps: From<B>,
    Steps: From<C>,
    Steps: From<D>,
{
    fn from((a, b, c, d): (A, B, C, D)) -> Self {
        Steps::from(a) + Steps::from(b) + Steps::from(c) + Steps::from(d)
    }
}
