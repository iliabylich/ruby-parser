use crate::{
    builder::{
        helpers::{collection_map, maybe_boxed_node_expr, maybe_loc, maybe_node_expr},
        Builder, Constructor,
    },
    nodes::{Begin, Ensure, Rescue, RescueBody},
    token::Token,
    Node,
};

impl<C: Constructor> Builder<C> {
    pub(crate) fn rescue_body(
        rescue_t: Token,
        exc_list: Vec<Node>,
        assoc_t_and_exc_var: Option<(Token, Box<Node>)>,
        then_t: Option<Token>,
        body: Option<Box<Node>>,
    ) -> Box<Node> {
        let exc_list = if exc_list.is_empty() {
            None
        } else {
            Some(Self::array(None, exc_list, None))
        };
        let (assoc_t, exc_var) = match assoc_t_and_exc_var {
            Some((assoc_t, exc_var)) => (Some(assoc_t), Some(exc_var)),
            None => (None, None),
        };

        let end_l = maybe_boxed_node_expr(&body)
            .or_else(|| maybe_loc(&then_t))
            .or_else(|| maybe_boxed_node_expr(&exc_var))
            .or_else(|| maybe_boxed_node_expr(&exc_list))
            .unwrap_or_else(|| rescue_t.loc);

        let expression_l = rescue_t.loc.join(&end_l);
        let keyword_l = rescue_t.loc;
        let assoc_l = maybe_loc(&assoc_t);
        let begin_l = maybe_loc(&then_t);

        Box::new(Node::RescueBody(RescueBody {
            exc_list,
            exc_var,
            body,
            keyword_l,
            assoc_l,
            begin_l,
            expression_l,
        }))
    }

    pub(crate) fn begin_body(
        compound_stmt: Option<Box<Node>>,
        rescue_bodies: Vec<Node>,
        opt_else: Option<(Token, Option<Box<Node>>)>,
        opt_ensure: Option<(Token, Option<Box<Node>>)>,
    ) -> Option<Box<Node>> {
        let mut result: Option<Box<Node>>;

        if !rescue_bodies.is_empty() {
            if let Some((else_t, else_)) = opt_else {
                let begin_l = maybe_boxed_node_expr(&compound_stmt)
                    .or_else(|| maybe_node_expr(&rescue_bodies.first()))
                    .unwrap_or_else(|| unreachable!("can't compute begin_l"));

                let end_l = maybe_boxed_node_expr(&else_).unwrap_or_else(|| else_t.loc);

                let expression_l = begin_l.join(&end_l);
                let else_l = else_t.loc;

                result = Some(Box::new(Node::Rescue(Rescue {
                    body: compound_stmt,
                    rescue_bodies,
                    else_,
                    else_l: Some(else_l),
                    expression_l,
                })))
            } else {
                let begin_l = maybe_boxed_node_expr(&compound_stmt)
                    .or_else(|| maybe_node_expr(&rescue_bodies.first()))
                    .unwrap_or_else(|| unreachable!("can't compute begin_l"));

                let end_l = maybe_node_expr(&rescue_bodies.last())
                    .unwrap_or_else(|| unreachable!("can't compute end_l"));

                let expression_l = begin_l.join(&end_l);
                let else_l = None;

                result = Some(Box::new(Node::Rescue(Rescue {
                    body: compound_stmt,
                    rescue_bodies,
                    else_: None,
                    else_l,
                    expression_l,
                })))
            }
        } else if let Some((else_t, else_)) = opt_else {
            let mut statements = vec![];

            if let Some(compound_stmt) = compound_stmt {
                match *compound_stmt {
                    Node::Begin(Begin {
                        statements: stmts, ..
                    }) => statements = stmts,
                    other => statements.push(other),
                }
            }

            let parts = if else_.is_some() {
                vec![*else_.unwrap()]
            } else {
                vec![]
            };
            let (begin_l, end_l, expression_l) = collection_map(&Some(else_t), &parts, &None);

            statements.push(Node::Begin(Begin {
                statements: parts,
                begin_l,
                end_l,
                expression_l,
            }));

            let (begin_l, end_l, expression_l) = collection_map(&None, &statements, &None);

            result = Some(Box::new(Node::Begin(Begin {
                statements,
                begin_l,
                end_l,
                expression_l,
            })))
        } else {
            result = compound_stmt;
        }

        if let Some((ensure_t, ensure)) = opt_ensure {
            let ensure_body = ensure;
            let keyword_l = ensure_t.loc;

            let begin_l = maybe_boxed_node_expr(&result).unwrap_or_else(|| ensure_t.loc);

            let end_l = maybe_node_expr(&ensure_body.as_ref().map(|x| x.as_ref()))
                .unwrap_or_else(|| ensure_t.loc);

            let expression_l = begin_l.join(&end_l);

            result = Some(Box::new(Node::Ensure(Ensure {
                body: result,
                ensure: ensure_body,
                keyword_l,
                expression_l,
            })))
        }

        result
    }
}
