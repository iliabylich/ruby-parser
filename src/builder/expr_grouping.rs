use crate::{
    builder::{helpers::nodes_locs, Builder},
    nodes::Begin,
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

    pub(crate) fn begin(begin_t: Token, statements: Vec<Node>, end_t: Token) -> Box<Node> {
        let begin_l = begin_t.loc;
        let end_l = end_t.loc;
        let expression_l = begin_l.join(&end_l);
        Box::new(Node::Begin(Begin {
            statements,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l,
        }))
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
