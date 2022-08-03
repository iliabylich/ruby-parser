use crate::{
    builder::Builder,
    nodes::{Encoding, File, Line},
    token::Token,
    Node,
};

impl Builder {
    pub(crate) fn __line__(line_t: Token) -> Box<Node> {
        let loc = line_t.loc;
        Box::new(Node::Line(Line { expression_l: loc }))
    }

    pub(crate) fn __file__(file_t: Token) -> Box<Node> {
        let loc = file_t.loc;
        Box::new(Node::File(File { expression_l: loc }))
    }

    pub(crate) fn __encoding__(encoding_t: Token) -> Box<Node> {
        let loc = encoding_t.loc;
        Box::new(Node::Encoding(Encoding { expression_l: loc }))
    }
}
