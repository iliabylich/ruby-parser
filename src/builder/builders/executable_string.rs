use crate::{
    builder::{builders::helpers::*, Builder, Constructor},
    nodes::*,
    token::{Token, TokenKind},
    Node,
};

impl<C: Constructor> Builder<C> {
    pub(crate) fn xstring_compose(begin_t: Token, parts: Vec<Node>, end_t: Token) -> Box<Node> {
        let begin_l = begin_t.loc;
        let end_l = end_t.loc;

        if begin_t.is(TokenKind::tXHEREDOC_BEG) {
            let heredoc_body_l = collection_expr(&parts).unwrap_or(end_l);
            let heredoc_end_l = end_l;
            let expression_l = begin_l;

            Box::new(Node::XHeredoc(XHeredoc {
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            }))
        } else {
            let expression_l = begin_l.join(&end_l);

            Box::new(Node::Xstr(Xstr {
                parts,
                begin_l,
                end_l,
                expression_l,
            }))
        }
    }
}
