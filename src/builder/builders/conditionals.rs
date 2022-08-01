use crate::{
    builder::{Builder, Constructor},
    token::Token,
    Node,
};

impl<C: Constructor> Builder<C> {
    pub(crate) fn condition(
        cond_t: Token,
        cond: Box<Node>,
        then_t: Token,
        if_true: Option<Box<Node>>,
        else_t: Option<Token>,
        if_false: Option<Box<Node>>,
        end_t: Option<Token>,
    ) -> Box<Node> {
        todo!("condition")
    }

    pub(crate) fn condition_mod(
        if_true: Option<Box<Node>>,
        if_false: Option<Box<Node>>,
        cond_t: Token,
        cond: Box<Node>,
    ) -> Box<Node> {
        todo!("condition_mod")
    }

    pub(crate) fn ternary(
        cond: Box<Node>,
        question_t: Token,
        if_true: Box<Node>,
        colon_t: Token,
        if_false: Box<Node>,
    ) -> Box<Node> {
        todo!("ternary")
    }
}
