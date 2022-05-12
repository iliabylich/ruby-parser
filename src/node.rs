#[derive(PartialEq, Debug)]
pub enum Node<'a> {
    Number(&'a str),
    Plus(Box<Node<'a>>, Box<Node<'a>>),
    Minus(Box<Node<'a>>, Box<Node<'a>>),
    Mult(Box<Node<'a>>, Box<Node<'a>>),
    Div(Box<Node<'a>>, Box<Node<'a>>),
    Pow(Box<Node<'a>>, Box<Node<'a>>),
    Parenthesized(Box<Node<'a>>),
}

impl<'a> Node<'a> {
    pub fn eval(&self) -> u32 {
        match self {
            Node::Number(n) => n.parse().unwrap(),
            Node::Plus(lhs, rhs) => lhs.eval() + rhs.eval(),
            Node::Minus(lhs, rhs) => lhs.eval() - rhs.eval(),
            Node::Mult(lhs, rhs) => lhs.eval() * rhs.eval(),
            Node::Div(lhs, rhs) => lhs.eval() / rhs.eval(),
            Node::Pow(lhs, rhs) => lhs.eval().pow(rhs.eval()),
            Node::Parenthesized(inner) => inner.eval(),
        }
    }
}
