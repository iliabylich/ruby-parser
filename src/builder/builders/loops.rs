use crate::{
    builder::{Builder, Constructor},
    Node, Token,
};

pub(crate) enum LoopType {}

impl<C: Constructor> Builder<C> {
    pub(crate) fn loop_(
        &self,
        loop_type: LoopType,
        keyword_t: Token,
        cond: Box<Node>,
        do_t: Token,
        body: Option<Box<Node>>,
        end_t: Token,
    ) -> Box<Node> {
        todo!("builder.loop")
    }

    pub(crate) fn loop_mod(
        &self,
        loop_type: LoopType,
        body: Box<Node>,
        keyword_t: Token,
        cond: Box<Node>,
    ) -> Box<Node> {
        todo!("builder.loop_mod")
    }

    pub(crate) fn for_(
        &self,
        for_t: Token,
        iterator: Box<Node>,
        in_t: Token,
        iteratee: Box<Node>,
        do_t: Token,
        body: Option<Box<Node>>,
        end_t: Token,
    ) -> Box<Node> {
        todo!("builder.for")
    }
}
