use crate::{builder::ArgsType, Node, Token};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum StepData {
    Node(Box<Node>),
    Token(Token),
    ArgsType(ArgsType),
}
impl From<Box<Node>> for StepData {
    fn from(node: Box<Node>) -> Self {
        Self::Node(node)
    }
}
impl From<Token> for StepData {
    fn from(token: Token) -> Self {
        Self::Token(token)
    }
}
impl From<ArgsType> for StepData {
    fn from(args_type: ArgsType) -> Self {
        Self::ArgsType(args_type)
    }
}
