use crate::{
    builder::{helpers::nodes_locs, Builder},
    nodes::{Begin, Mlhs},
    token::Token,
    Node,
};

impl Builder {
    pub(crate) fn compstmt(statements: Vec<Node>) -> Box<Node> {
        debug_assert!(!statements.is_empty());

        if statements.len() == 1 {
            Box::new(statements.into_iter().next().unwrap())
        } else {
            let (begin_l, end_l, expression_l) = nodes_locs(&statements);

            Box::new(Node::Begin(Begin {
                statements,
                begin_l: Some(begin_l),
                end_l: Some(end_l),
                expression_l,
            }))
        }
    }

    pub(crate) fn begin(begin_t: Token, body: Option<Box<Node>>, end_t: Token) -> Box<Node> {
        let new_begin_l = begin_t.loc;
        let new_end_l = end_t.loc;
        let new_expression_l = new_begin_l.join(&new_end_l);

        let new_begin_l = Some(new_begin_l);
        let new_end_l = Some(new_end_l);

        if let Some(body) = body {
            let mut body = *body;
            match &mut body {
                Node::Mlhs(Mlhs {
                    begin_l,
                    end_l,
                    expression_l,
                    ..
                }) => {
                    // Synthesized (begin) from compstmt "a; b" or (mlhs)
                    // from multi_lhs "(a, b) = *foo".
                    *begin_l = new_begin_l;
                    *end_l = new_end_l;
                    *expression_l = new_expression_l;
                    Box::new(body)
                }
                Node::Begin(Begin {
                    begin_l,
                    end_l,
                    expression_l,
                    ..
                }) if begin_l.is_none() && end_l.is_none() => {
                    *begin_l = new_begin_l;
                    *end_l = new_end_l;
                    *expression_l = new_expression_l;
                    Box::new(body)
                }
                _ => Box::new(Node::Begin(Begin {
                    statements: vec![body],
                    begin_l: new_begin_l,
                    end_l: new_end_l,
                    expression_l: new_expression_l,
                })),
            }
        } else {
            // A nil expression: `()'.
            Box::new(Node::Begin(Begin {
                statements: vec![],
                begin_l: new_begin_l,
                end_l: new_end_l,
                expression_l: new_expression_l,
            }))
        }
    }

    pub(crate) fn group(nodes: Vec<Node>) -> Box<Node> {
        debug_assert!(nodes.len() > 0);

        if nodes.len() == 1 {
            return Box::new(nodes.into_iter().next().unwrap());
        }

        let (_, _, expression_l) = nodes_locs(&nodes);

        Box::new(Node::Begin(Begin {
            statements: nodes,
            begin_l: None,
            end_l: None,
            expression_l,
        }))
    }

    pub(crate) fn group_with_trailing_comma(nodes: Vec<Node>, trailing_comma: Token) -> Box<Node> {
        debug_assert!(nodes.len() > 0);

        let (_, _, mut expression_l) = nodes_locs(&nodes);
        expression_l = expression_l.join(&trailing_comma.loc);

        Box::new(Node::Begin(Begin {
            statements: nodes,
            begin_l: None,
            end_l: None,
            expression_l,
        }))
    }
}
