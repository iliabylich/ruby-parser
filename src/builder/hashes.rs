use crate::{
    buffer::Buffer,
    builder::{
        helpers::{collection_map, string_value},
        Builder,
    },
    nodes::{
        Complex, Const, Dstr, Dsym, Float, Hash, Int, Kwsplat, Lvar, Pair, Rational, Regexp, Str,
        Sym,
    },
    Node, Token,
};

impl Builder {
    pub(crate) fn pair(key: Box<Node>, assoc_t: Token, value: Box<Node>) -> Box<Node> {
        let operator_l = assoc_t.loc;
        let expression_l = key.expression().join(value.expression());

        Box::new(Node::Pair(Pair {
            key,
            value,
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn pair_keyword(key_t: Token, value: Box<Node>, buffer: &Buffer) -> Box<Node> {
        let key_loc = key_t.loc;
        let key_l = key_loc.adjust_end(-1);
        let colon_l = key_loc.with_start(key_loc.end - 1);
        let expression_l = key_loc.join(value.expression());

        let key = string_value(key_l, buffer);
        // self.validate_sym_value(&key, &key_l);

        Box::new(Node::Pair(Pair {
            key: Box::new(Node::Sym(Sym {
                name: key,
                begin_l: None,
                end_l: None,
                expression_l: key_l,
            })),
            value,
            operator_l: colon_l,
            expression_l,
        }))
    }

    pub(crate) fn pair_quoted(mut key: Box<Node>, colon_t: Token, value: Box<Node>) -> Box<Node> {
        let colon_l = colon_t.loc;
        let expression_l = key.expression().join(value.expression());

        match *key {
            Node::Str(Str {
                value,
                begin_l,
                end_l,
                expression_l,
            }) => {
                key = Box::new(Node::Sym(Sym {
                    name: value,
                    begin_l,
                    end_l,
                    expression_l,
                }))
            }
            Node::Dstr(Dstr {
                parts,
                begin_l,
                end_l,
                expression_l,
            }) => {
                key = Box::new(Node::Dsym(Dsym {
                    parts,
                    begin_l,
                    end_l,
                    expression_l,
                }))
            }
            _ => unreachable!(),
        }

        Box::new(Node::Pair(Pair {
            key,
            value,
            operator_l: colon_l,
            expression_l,
        }))
    }

    pub(crate) fn pair_label(key_t: Token, buffer: &Buffer) -> Box<Node> {
        let key_l = key_t.loc;
        let value_l = key_l.adjust_end(-1);

        let label = string_value(value_l, buffer);
        let value = if label
            .as_str()
            .chars()
            .next()
            .expect("bug: label can't be empty")
            .is_lowercase()
        {
            Box::new(Node::Lvar(Lvar {
                name: label,
                expression_l: value_l,
            }))
        } else {
            Box::new(Node::Const(Const {
                scope: None,
                name: label,
                double_colon_l: None,
                name_l: value_l,
                expression_l: value_l,
            }))
        };

        Self::pair_keyword(key_t, Self::accessible(value), buffer)
    }

    pub(crate) fn kwsplat(dstar_t: Token, value: Box<Node>) -> Box<Node> {
        let operator_l = dstar_t.loc;
        let expression_l = value.expression().join(&operator_l);

        Box::new(Node::Kwsplat(Kwsplat {
            value,
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn associate(
        begin_t: Option<Token>,
        pairs: Vec<Node>,
        end_t: Option<Token>,
    ) -> Box<Node> {
        for i in 0..pairs.len() {
            for j in i + 1..pairs.len() {
                let key1 = if let Node::Pair(Pair { key, .. }) = &pairs[i] {
                    &**key
                } else {
                    // kwsplat
                    continue;
                };
                let key2 = if let Node::Pair(Pair { key, .. }) = &pairs[j] {
                    &**key
                } else {
                    // kwsplat
                    continue;
                };

                fn keys_are_equal(left: &Node, right: &Node) -> bool {
                    match (left, right) {
                        // sym
                        (
                            Node::Sym(Sym { name: name1, .. }),
                            Node::Sym(Sym { name: name2, .. }),
                        ) if name1 == name2 => true,

                        // str
                        (
                            Node::Str(Str { value: value1, .. }),
                            Node::Str(Str { value: value2, .. }),
                        ) if value1 == value2 => true,

                        // int
                        (
                            Node::Int(Int { value: value1, .. }),
                            Node::Int(Int { value: value2, .. }),
                        ) if value1 == value2 => true,

                        // float
                        (
                            Node::Float(Float { value: value1, .. }),
                            Node::Float(Float { value: value2, .. }),
                        ) if value1 == value2 => true,

                        // rational
                        (
                            Node::Rational(Rational { value: value1, .. }),
                            Node::Rational(Rational { value: value2, .. }),
                        ) if value1 == value2 => true,

                        // complex
                        (
                            Node::Complex(Complex { value: value1, .. }),
                            Node::Complex(Complex { value: value2, .. }),
                        ) if value1 == value2 => true,

                        // regexp
                        (
                            Node::Regexp(Regexp {
                                parts: parts1,
                                options: options1,
                                ..
                            }),
                            Node::Regexp(Regexp {
                                parts: parts2,
                                options: options2,
                                ..
                            }),
                        ) if options1 == options2 => {
                            parts1.len() == parts2.len()
                                && parts1
                                    .iter()
                                    .zip(parts2.iter())
                                    .all(|(child1, child2)| keys_are_equal(child1, child2))
                        }

                        _ => false,
                    }
                }

                let do_warn = keys_are_equal(key1, key2);

                if do_warn {
                    // self.warn(DiagnosticMessage::DuplicateHashKey {}, key2.expression());
                }
            }
        }

        let (begin_l, end_l, expression_l) = collection_map(&begin_t, &pairs, &end_t);

        Box::new(Node::Hash(Hash {
            pairs,
            begin_l,
            end_l,
            expression_l,
        }))
    }
}
