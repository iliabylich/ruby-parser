#[derive(PartialEq, Debug)]
pub enum Node {
    Number(u32),
    Plus(Box<Node>, Box<Node>),
    Minus(Box<Node>, Box<Node>),
    Mult(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Parenthesized(Box<Node>),
}

impl Node {
    pub fn eval(&self) -> u32 {
        match self {
            Node::Number(n) => *n,
            Node::Plus(lhs, rhs) => lhs.eval() + rhs.eval(),
            Node::Minus(lhs, rhs) => lhs.eval() - rhs.eval(),
            Node::Mult(lhs, rhs) => lhs.eval() * rhs.eval(),
            Node::Div(lhs, rhs) => lhs.eval() / rhs.eval(),
            Node::Pow(lhs, rhs) => lhs.eval().pow(rhs.eval()),
            Node::Parenthesized(inner) => inner.eval(),
        }
    }
}
