use crate::{
    builder::{
        helpers::{maybe_boxed_node_expr, maybe_loc},
        Builder, Constructor,
    },
    nodes::{If, IfMod, IfTernary},
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
        let end_l = maybe_loc(&end_t)
            .or_else(|| maybe_boxed_node_expr(&if_false))
            .or_else(|| maybe_loc(&else_t))
            .or_else(|| maybe_boxed_node_expr(&if_true))
            .unwrap_or_else(|| then_t.loc);

        let expression_l = cond_t.loc.join(&end_l);
        let keyword_l = cond_t.loc;
        let begin_l = then_t.loc;
        let else_l = maybe_loc(&else_t);
        let end_l = maybe_loc(&end_t);

        // TODO: call check_condition on cond
        Box::new(Node::If(If {
            cond,
            if_true,
            if_false,
            keyword_l,
            begin_l,
            else_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn condition_mod(
        if_true: Option<Box<Node>>,
        if_false: Option<Box<Node>>,
        cond_t: Token,
        cond: Box<Node>,
    ) -> Box<Node> {
        let pre = match (if_true.as_ref(), if_false.as_ref()) {
            (None, None) => unreachable!("at least one of if_true/if_false is required"),
            (None, Some(if_false)) => if_false,
            (Some(if_true), None) => if_true,
            (Some(_), Some(_)) => unreachable!("only one of if_true/if_false is required"),
        };

        let expression_l = pre.expression().join(cond.expression());
        let keyword_l = cond_t.loc;

        // TODO: call check_condition on cond
        Box::new(Node::IfMod(IfMod {
            cond,
            if_true,
            if_false,
            keyword_l,
            expression_l,
        }))
    }

    pub(crate) fn ternary(
        cond: Box<Node>,
        question_t: Token,
        if_true: Box<Node>,
        colon_t: Token,
        if_false: Box<Node>,
    ) -> Box<Node> {
        let expression_l = cond.expression().join(if_false.expression());
        let question_l = question_t.loc;
        let colon_l = colon_t.loc;

        Box::new(Node::IfTernary(IfTernary {
            cond,
            if_true,
            if_false,
            question_l,
            colon_l,
            expression_l,
        }))
    }
}
