use crate::{
    builder::{Builder, Constructor},
    Node, Token,
};

impl<C: Constructor> Builder<C> {
    pub(crate) fn range_inclusive(
        left: Option<Box<Node>>,
        dot2_t: Token,
        right: Option<Box<Node>>,
    ) -> Box<Node> {
        todo!("builder.range_inclusive")
    }

    pub(crate) fn range_exclusive(
        left: Option<Box<Node>>,
        dot3_t: Token,
        right: Option<Box<Node>>,
    ) -> Box<Node> {
        todo!("builder.range_exclusive")
    }
}
