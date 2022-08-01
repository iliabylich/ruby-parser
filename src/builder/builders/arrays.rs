use crate::{
    builder::{builders::helpers::*, Builder, Constructor},
    nodes::*,
    token::Token,
    Node,
};

impl<C: Constructor> Builder<C> {
    pub(crate) fn array(
        begin_t: Option<Token>,
        elements: Vec<Node>,
        end_t: Option<Token>,
    ) -> Box<Node> {
        let (begin_l, end_l, expression_l) = collection_map(&begin_t, &elements, &end_t);

        Box::new(Node::Array(Array {
            elements,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn splat(star_t: Token, value: Box<Node>) -> Box<Node> {
        let operator_l = star_t.loc;
        let expression_l = operator_l.join(value.expression());
        Box::new(Node::Splat(Splat {
            value: Some(value),
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn nameless_splat(star_t: Token) -> Box<Node> {
        let operator_l = star_t.loc;
        let expression_l = operator_l;
        Box::new(Node::Splat(Splat {
            value: None,
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn word(parts: Vec<Node>) -> Box<Node> {
        debug_assert!(!parts.is_empty());

        if parts.len() == 1 && matches!(&parts[0], Node::Str(_) | Node::Dstr(_)) {
            let part = parts.into_iter().next().expect("expected 1 element");
            return Box::new(part);
        }

        let (begin_l, end_l, expression_l) = collection_map(&None, &parts, &None);
        Box::new(Node::Dstr(Dstr {
            parts,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn words_compose(begin_t: Token, elements: Vec<Node>, end_t: Token) -> Box<Node> {
        let begin_l = begin_t.loc;
        let end_l = end_t.loc;
        let expression_l = begin_l.join(&end_l);
        Box::new(Node::Array(Array {
            elements,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l,
        }))
    }

    pub(crate) fn symbols_compose(begin_t: Token, elements: Vec<Node>, end_t: Token) -> Box<Node> {
        let elements = elements
            .into_iter()
            .map(|part| match part {
                Node::Str(Str {
                    value,
                    begin_l,
                    end_l,
                    expression_l,
                }) => {
                    // TODO: validate_sym_value
                    Node::Sym(Sym {
                        name: value,
                        begin_l,
                        end_l,
                        expression_l,
                    })
                }
                Node::Dstr(Dstr {
                    parts,
                    begin_l,
                    end_l,
                    expression_l,
                }) => Node::Dsym(Dsym {
                    parts,
                    begin_l,
                    end_l,
                    expression_l,
                }),
                other => other,
            })
            .collect::<Vec<_>>();

        let begin_l = begin_t.loc;
        let end_l = end_t.loc;
        let expression_l = begin_l.join(&end_l);
        Box::new(Node::Array(Array {
            elements,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l,
        }))
    }
}
