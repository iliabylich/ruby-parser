use crate::{
    buffer::Buffer,
    builder::{builders::helpers::string_value, Builder},
    nodes::{Dsym, Str, Sym},
    token::Token,
    Node,
};

impl Builder {
    pub(crate) fn symbol(start_t: Token, value_t: Token, buffer: &Buffer) -> Box<Node> {
        let begin_l = start_t.loc;
        let value_l = value_t.loc;
        let expression_l = begin_l.join(&value_l);
        let value = string_value(value_l, buffer);
        // TODO: validate_sym_value
        Box::new(Node::Sym(Sym {
            name: value,
            begin_l: Some(begin_l),
            end_l: None,
            expression_l,
        }))
    }

    pub(crate) fn symbol_internal(symbol_t: Token, buffer: &Buffer) -> Box<Node> {
        let expression_l = symbol_t.loc;
        let value = string_value(expression_l, buffer);
        // TODO: validate_sym_value
        Box::new(Node::Sym(Sym {
            name: value,
            begin_l: None,
            end_l: None,
            expression_l,
        }))
    }

    pub(crate) fn symbol_compose(begin_t: Token, parts: Vec<Node>, end_t: Token) -> Box<Node> {
        let begin_l = begin_t.loc;
        let end_l = end_t.loc;
        let expression_l = begin_l.join(&end_l);

        if parts.len() == 1 && matches!(&parts[0], Node::Str(_)) {
            let part = parts.into_iter().next().unwrap();
            let value = if let Node::Str(Str { value, .. }) = part {
                value
            } else {
                unreachable!()
            };

            // TODO: validate_sym_value

            return Box::new(Node::Sym(Sym {
                name: value,
                begin_l: Some(begin_l),
                end_l: Some(end_l),
                expression_l,
            }));
        }

        Box::new(Node::Dsym(Dsym {
            parts,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l,
        }))
    }
}
