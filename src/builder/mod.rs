use crate::{nodes::*, Node, Token};

pub(crate) fn gvar<'a>(token: Token<'a>) -> Box<Node<'a>> {
    let expression_l = token.loc();
    Box::new(Node::Gvar(Gvar {
        name: String::from("foo"),
        expression_l,
    }))
}

pub(crate) fn alias<'a>(
    alias_t: Token<'a>,
    to: Box<Node<'a>>,
    from: Box<Node<'a>>,
) -> Box<Node<'a>> {
    let keyword_l = alias_t.loc();
    let expression_l = keyword_l.join(from.expression());
    Box::new(Node::Alias(Alias {
        to,
        from,
        keyword_l,
        expression_l,
    }))
}
