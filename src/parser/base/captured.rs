use crate::{Node, Token};

#[derive(Debug)]
pub(crate) enum CapturedItem {
    Node(Node),
    Token(Token),
}

#[derive(Debug, Default)]
pub(crate) struct Captured {
    items: Vec<CapturedItem>,
}

impl From<Node> for Captured {
    fn from(node: Node) -> Self {
        Self {
            items: vec![CapturedItem::Node(node)],
        }
    }
}

impl From<Token> for Captured {
    fn from(token: Token) -> Self {
        Self {
            items: vec![CapturedItem::Token(token)],
        }
    }
}

impl<T> From<Vec<T>> for Captured
where
    Captured: From<T>,
{
    fn from(v: Vec<T>) -> Self {
        v.into_iter()
            .map(|v| Captured::from(v))
            .reduce(|acc, v| acc + v)
            .unwrap_or_default()
    }
}

impl std::ops::Add for Captured {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.items.append(&mut rhs.items);
        self
    }
}
