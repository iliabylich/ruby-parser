use crate::{
    builder::{
        helpers::{maybe_boxed_node_expr, maybe_loc, maybe_node_expr},
        Builder,
    },
    nodes::{Case, When},
    Node, Token,
};

impl Builder {
    pub(crate) fn when(
        when_t: Token,
        patterns: Vec<Node>,
        then_t: Token,
        body: Option<Box<Node>>,
    ) -> Box<Node> {
        let begin_l = then_t.loc;

        let expr_end_l = maybe_boxed_node_expr(&body)
            .or_else(|| maybe_node_expr(&patterns.last()))
            .unwrap_or_else(|| when_t.loc);
        let when_l = when_t.loc;
        let expression_l = when_l.join(&expr_end_l);

        Box::new(Node::When(When {
            patterns,
            body,
            keyword_l: when_l,
            begin_l,
            expression_l,
        }))
    }

    pub(crate) fn case(
        case_t: Token,
        expr: Option<Box<Node>>,
        when_bodies: Vec<Node>,
        else_t: Option<Token>,
        else_body: Option<Box<Node>>,
        end_t: Token,
    ) -> Box<Node> {
        let keyword_l = case_t.loc;
        let else_l = maybe_loc(&else_t);
        let end_l = end_t.loc;
        let expression_l = keyword_l.join(&end_l);

        Box::new(Node::Case(Case {
            expr,
            when_bodies,
            else_body,
            keyword_l,
            else_l,
            end_l,
            expression_l,
        }))
    }
}
