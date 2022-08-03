use crate::{
    builder::Builder,
    nodes::{Postexe, Preexe},
    token::Token,
    Node,
};

impl Builder {
    pub(crate) fn preexe(
        preexe_t: Token,
        lcurly_t: Token,
        body: Option<Box<Node>>,
        rcurly_t: Token,
    ) -> Box<Node> {
        let keyword_l = preexe_t.loc;
        let begin_l = lcurly_t.loc;
        let end_l = rcurly_t.loc;
        let expression_l = keyword_l.join(&end_l);

        Box::new(Node::Preexe(Preexe {
            body,
            keyword_l,
            begin_l,
            end_l,
            expression_l,
        }))
    }
    pub(crate) fn postexe(
        postexe_t: Token,
        lcurly_t: Token,
        body: Option<Box<Node>>,
        rcurly_t: Token,
    ) -> Box<Node> {
        let keyword_l = postexe_t.loc;
        let begin_l = lcurly_t.loc;
        let end_l = rcurly_t.loc;
        let expression_l = keyword_l.join(&end_l);

        Box::new(Node::Postexe(Postexe {
            body,
            keyword_l,
            begin_l,
            end_l,
            expression_l,
        }))
    }
}
