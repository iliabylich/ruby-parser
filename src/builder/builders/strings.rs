use crate::{
    buffer::Buffer,
    builder::{
        builders::helpers::{collection_map, heredoc_map, is_heredoc, string_value},
        Builder, Constructor,
    },
    loc::loc,
    nodes::{Dstr, Heredoc, Str},
    string_content::StringContent,
    token::{Token, TokenKind},
    Node,
};

impl<C: Constructor> Builder<C> {
    pub(crate) fn str_node(
        begin_t: Option<Token>,
        value: StringContent,
        parts: Vec<Node>,
        end_t: Option<Token>,
    ) -> Box<Node> {
        if let Some(Token {
            kind: TokenKind::tHEREDOC_BEG,
            ..
        }) = &begin_t
        {
            let (heredoc_body_l, heredoc_end_l, expression_l) =
                heredoc_map(&begin_t, &parts, &end_t);

            Box::new(Node::Heredoc(Heredoc {
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            }))
        } else {
            let (begin_l, end_l, expression_l) = collection_map(&begin_t, &parts, &end_t);

            Box::new(Node::Str(Str {
                value,
                begin_l,
                end_l,
                expression_l,
            }))
        }
    }

    pub(crate) fn string_internal(string_t: Token, buffer: &Buffer) -> Box<Node> {
        let expression_l = string_t.loc;
        let value = string_value(expression_l, buffer);
        Box::new(Node::Str(Str {
            value,
            begin_l: None,
            end_l: None,
            expression_l,
        }))
    }

    pub(crate) fn string_compose(
        begin_t: Option<Token>,
        parts: Vec<Node>,
        end_t: Option<Token>,
    ) -> Box<Node> {
        match &parts[..] {
            [] => {
                return Self::str_node(begin_t, StringContent::from(""), parts, end_t);
            }

            [Node::Str(_) | Node::Dstr(_) | Node::Heredoc(_)]
                if begin_t.is_none() && end_t.is_none() =>
            {
                return Box::new(parts.into_iter().next().expect("expected at least 1 item"));
            }

            [Node::Str(Str { value, .. })] => {
                return Self::str_node(begin_t, value.clone(), parts, end_t);
            }

            [Node::Dstr(_) | Node::Heredoc(_)] => {
                unreachable!("dstr or heredoc string without begin_t/end_t")
            }

            _ => {}
        }

        if is_heredoc(&begin_t) {
            let (heredoc_body_l, heredoc_end_l, expression_l) =
                heredoc_map(&begin_t, &parts, &end_t);

            Box::new(Node::Heredoc(Heredoc {
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            }))
        } else {
            let (begin_l, end_l, expression_l) = collection_map(&begin_t, &parts, &end_t);

            Box::new(Node::Dstr(Dstr {
                parts,
                begin_l,
                end_l,
                expression_l,
            }))
        }
    }

    pub(crate) fn character(char_t: Token) -> Box<Node> {
        let expression_l = char_t.loc;
        let begin_l = loc!(expression_l.start, expression_l.start + 1);

        let value = if let Token {
            kind: TokenKind::tCHAR,
            value: Some(value),
            ..
        } = char_t
        {
            value
        } else {
            unreachable!()
        };

        let value = StringContent::from(value);
        Box::new(Node::Str(Str {
            value,
            begin_l: Some(begin_l),
            end_l: None,
            expression_l,
        }))
    }
}
