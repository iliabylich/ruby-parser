use crate::{
    builder::{Builder, Constructor},
    Node, Token,
};

impl<C: Constructor> Builder<C> {
    pub(crate) fn args(
        begin_t: Option<Token>,
        args: Vec<Node>,
        end_t: Option<Token>,
    ) -> Option<Box<Node>> {
        todo!("builder.args")
    }

    pub(crate) fn forward_arg(dots_t: Token) -> Box<Node> {
        todo!("builder.forward_arg")
    }

    pub(crate) fn arg(name_t: Token) -> Box<Node> {
        todo!("builder.arg")
    }

    pub(crate) fn optarg(name_t: Token, eql_t: Token, default: Box<Node>) -> Box<Node> {
        todo!("builder.optarg")
    }

    pub(crate) fn restarg(star_t: Token, name_t: Option<Token>) -> Box<Node> {
        todo!("builder.restarg")
    }

    pub(crate) fn kwarg(name_t: Token) -> Box<Node> {
        todo!("builder.kwarg")
    }

    pub(crate) fn kwoptarg(name_t: Token, default: Box<Node>) -> Box<Node> {
        todo!("builder.kwoptarg")
    }

    pub(crate) fn kwrestarg(dstar_t: Token, name_t: Option<Token>) -> Box<Node> {
        todo!("builder.kwrestarg")
    }

    pub(crate) fn kwnilarg(dstar_t: Token, nil_t: Token) -> Box<Node> {
        todo!("builder.kwnilarg")
    }

    pub(crate) fn shadowarg(name_t: Token) -> Box<Node> {
        todo!("builder.shadowarg")
    }

    pub(crate) fn blockarg(amper_t: Token, name_t: Option<Token>) -> Box<Node> {
        todo!("builder.blockarg")
    }

    pub(crate) fn procarg0(arg: Box<Node>) -> Box<Node> {
        todo!("builder.procarg0")
    }
}
