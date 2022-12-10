use crate::{Node, Token};

pub(crate) trait Unbox {
    type Output;

    fn unbox(self) -> Self::Output
    where
        Self: Sized;
}

impl Unbox for Node {
    type Output = Node;

    fn unbox(self) -> Self::Output
    where
        Self: Sized,
    {
        self
    }
}

impl Unbox for Box<Node> {
    type Output = Node;

    fn unbox(self) -> Self::Output
    where
        Self: Sized,
    {
        *self
    }
}

impl Unbox for Token {
    type Output = Token;

    fn unbox(self) -> Self::Output
    where
        Self: Sized,
    {
        self
    }
}

impl Unbox for () {
    type Output = ();

    fn unbox(self) -> Self::Output
    where
        Self: Sized,
    {
        ()
    }
}
