use crate::{
    builder::{Builder, Constructor},
    nodes::{For, Until, UntilPost, While, WhilePost},
    Node, Token,
};

pub(crate) enum LoopType {
    While,
    Until,
}

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
        let keyword_l = keyword_t.loc;
        let begin_l = do_t.loc;
        let end_l = end_t.loc;
        let expression_l = keyword_t.loc.join(&end_l);

        // let cond = self.check_condition(cond);

        match loop_type {
            LoopType::While => Box::new(Node::While(While {
                cond,
                body,
                keyword_l,
                begin_l: Some(begin_l),
                end_l: Some(end_l),
                expression_l,
            })),
            LoopType::Until => Box::new(Node::Until(Until {
                cond,
                body,
                keyword_l,
                begin_l: Some(begin_l),
                end_l: Some(end_l),
                expression_l,
            })),
        }
    }

    pub(crate) fn loop_mod(
        &self,
        loop_type: LoopType,
        body: Box<Node>,
        keyword_t: Token,
        cond: Box<Node>,
    ) -> Box<Node> {
        let expression_l = body.expression().join(cond.expression());
        let keyword_l = keyword_t.loc;

        // let cond = self.check_condition(cond);

        match (loop_type, &*body) {
            (LoopType::While, Node::KwBegin(_)) => Box::new(Node::WhilePost(WhilePost {
                cond,
                body,
                keyword_l,
                expression_l,
            })),
            (LoopType::While, _) => Box::new(Node::While(While {
                cond,
                body: Some(body),
                keyword_l,
                begin_l: None,
                end_l: None,
                expression_l,
            })),
            (LoopType::Until, Node::KwBegin(_)) => Box::new(Node::UntilPost(UntilPost {
                cond,
                body,
                keyword_l,
                expression_l,
            })),
            (LoopType::Until, _) => Box::new(Node::Until(Until {
                cond,
                body: Some(body),
                keyword_l,
                begin_l: None,
                end_l: None,
                expression_l,
            })),
        }
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
        let keyword_l = for_t.loc;
        let operator_l = in_t.loc;
        let begin_l = do_t.loc;
        let end_l = end_t.loc;
        let expression_l = keyword_l.join(&end_l);

        Box::new(Node::For(For {
            iterator,
            iteratee,
            body,
            keyword_l,
            operator_l,
            begin_l,
            end_l,
            expression_l,
        }))
    }
}
