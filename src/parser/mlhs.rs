use crate::{
    builder::{Builder, Constructor},
    nodes::Node,
    parser::Parser,
    token::TokenValue,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn parse_mlhs(&mut self) -> MLHS<'a> {
        let mut items = vec![];
        let mut has_splat = false;
        let mut definitely_mlhs = false;
        let mut has_trailing_comma = false;

        loop {
            match self.parse_mlhs_item() {
                MLHS::DefinitelyMlhs { node } => {
                    definitely_mlhs = true;

                    if let Node::Splat(_) = &*node {
                        if has_splat {
                            panic!("two splats in mlhs on the same level")
                        }
                        has_splat = true;
                    }

                    items.push(*node);
                }

                MLHS::MaybeLhs { node } => {
                    if let Node::Splat(_) = &*node {
                        if has_splat {
                            panic!("two splats in mlhs on the same level")
                        }
                        has_splat = true;
                    }

                    items.push(*node);
                }

                MLHS::None => break,
            }

            has_trailing_comma = false;
            if matches!(self.current_token().value(), TokenValue::tCOMMA) {
                // consume ,
                self.take_token();
                has_trailing_comma = true;
                definitely_mlhs = true;
            } else {
                break;
            }
        }

        if !has_trailing_comma && matches!(self.current_token().value(), TokenValue::tCOMMA) {
            // consume ,
            self.take_token();
        }

        if items.is_empty() {
            MLHS::None
        } else if definitely_mlhs {
            MLHS::DefinitelyMlhs {
                node: Builder::<C>::group(items),
            }
        } else {
            debug_assert_eq!(items.len(), 1);
            let node = items.into_iter().next().unwrap();
            MLHS::MaybeLhs {
                node: Box::new(node),
            }
        }
    }

    fn parse_mlhs_item(&mut self) -> MLHS<'a> {
        if let Some(lparen_t) = self.try_token(TokenValue::tLPAREN) {
            match self.parse_mlhs() {
                MLHS::DefinitelyMlhs { node: inner } => {
                    let rparen_t = self.expect_token(TokenValue::tRPAREN);
                    MLHS::DefinitelyMlhs {
                        node: Builder::<C>::begin(lparen_t, Some(inner), rparen_t),
                    }
                }
                MLHS::MaybeLhs { node: inner } => {
                    let rparen_t = self.expect_token(TokenValue::tRPAREN);
                    MLHS::MaybeLhs {
                        node: Builder::<C>::begin(lparen_t, Some(inner), rparen_t),
                    }
                }
                MLHS::None => MLHS::None,
            }
        } else if let Some(star_t) = self.try_token(TokenValue::tSTAR) {
            match self.parse_mlhs_primitive_item() {
                Some(node) => MLHS::DefinitelyMlhs {
                    node: Builder::<C>::splat(star_t, node),
                },
                None => match self.current_token().value() {
                    TokenValue::tCOMMA | TokenValue::tRPAREN => MLHS::DefinitelyMlhs {
                        node: Builder::<C>::nameless_splat(star_t),
                    },
                    _ => MLHS::None,
                },
            }
        } else {
            match self.parse_mlhs_primitive_item() {
                Some(node) => MLHS::MaybeLhs { node },
                None => MLHS::None,
            }
        }
    }

    fn parse_mlhs_primitive_item(&mut self) -> Option<Box<Node<'a>>> {
        let trivial = None
            .or_else(|| self.parse_user_variable())
            .or_else(|| self.parse_keyword_variable())
            .or_else(|| self.parse_user_variable());

        if let Some(node) = trivial {
            return Some(node);
        }

        if let Some(primary) = self.parse_primary() {
            if let Some(lbrack_t) = self.try_token(TokenValue::tLBRACK) {
                // foo[bar] = something
                let opt_call_args = self.parse_opt_call_args();
                let rbrack_t = self.expect_token(TokenValue::tRBRACK);
                todo!(
                    "return foo[bar] {:?} {:?} {:?} {:?}",
                    primary,
                    lbrack_t,
                    opt_call_args,
                    rbrack_t
                );
            }

            if let Some(colon2_t) = self.try_token(TokenValue::tCOLON2) {
                // Foo::Bar = something

                // TODO: or tIDENTIFIER
                let constant_t = self.expect_token(TokenValue::tCONSTANT);
                todo!(
                    "return Foo::Bar = {:?} {:?} {:?}",
                    primary,
                    colon2_t,
                    constant_t
                );
            }

            if let Some(call_op_t) = self.parse_call_op() {
                // TODO: or tCONSTANT
                let call_mid = self.expect_token(TokenValue::tIDENTIFIER);
                todo!(
                    "return Foo.bar = {:?} {:?} {:?}",
                    primary,
                    call_op_t,
                    call_mid
                )
            }
        }

        None
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum MLHS<'a> {
    // This variant is used if there's at least 1 comma
    // i.e. `a, b` or `((a, b))`
    // even `a,` is an MLHS
    DefinitelyMlhs { node: Box<Node<'a>> },

    // This variant is used if it looks like an LHS
    // i.e. has no commas but is techincally assignable
    // like `((a))`
    MaybeLhs { node: Box<Node<'a>> },

    // This variant is used if we found something that is
    // absolutely not assignable
    // like `def foo; end`
    None,
}

enum MlhsInner<'a> {
    DefinitelyMlhsNode(Box<Node<'a>>),
    DefinitelyMlhsList(Vec<Node<'a>>),
    MaybeLhs(Box<Node<'a>>),
    None,
}

#[cfg(test)]
use crate::{loc::loc, parser::RustParser, string_content::StringContent};

#[test]
fn test_lhs_user_variable() {
    use crate::nodes::Lvar;

    let mut parser = RustParser::new(b"a");
    assert_eq!(
        parser.parse_mlhs(),
        MLHS::MaybeLhs {
            node: Box::new(Node::Lvar(Lvar {
                name: StringContent::from("a"),
                expression_l: loc!(0, 1)
            }))
        }
    );
}

#[test]
fn test_lhs_parenthesized() {
    use crate::nodes::{Begin, Lvar};

    let mut parser = RustParser::new(b"((a))");
    assert_eq!(
        parser.parse_mlhs(),
        MLHS::MaybeLhs {
            node: Box::new(Node::Begin(Begin {
                statements: vec![Node::Begin(Begin {
                    statements: vec![Node::Lvar(Lvar {
                        name: StringContent::from("a"),
                        expression_l: loc!(2, 3)
                    })],
                    begin_l: Some(loc!(1, 2)),
                    end_l: Some(loc!(3, 4)),
                    expression_l: loc!(1, 4)
                })],
                begin_l: Some(loc!(0, 1)),
                end_l: Some(loc!(4, 5)),
                expression_l: loc!(0, 5)
            }))
        }
    );
}

#[test]
fn test_mlhs_without_parens() {
    use crate::nodes::{Begin, Lvar, Splat};

    let mut parser = RustParser::new(b"a, *b, c");
    assert_eq!(
        parser.parse_mlhs(),
        MLHS::DefinitelyMlhs {
            node: Box::new(Node::Begin(Begin {
                statements: vec![
                    Node::Lvar(Lvar {
                        name: StringContent::from("a"),
                        expression_l: loc!(0, 1)
                    }),
                    Node::Splat(Splat {
                        value: Some(Box::new(Node::Lvar(Lvar {
                            name: StringContent::from("b"),
                            expression_l: loc!(4, 5)
                        }))),
                        operator_l: loc!(3, 4),
                        expression_l: loc!(3, 5)
                    }),
                    Node::Lvar(Lvar {
                        name: StringContent::from("c"),
                        expression_l: loc!(7, 8)
                    })
                ],
                begin_l: Some(loc!(0, 1)),
                end_l: Some(loc!(8, 9)),
                expression_l: loc!(0, 9)
            }))
        }
    );
}

#[test]
fn test_mlhs_with_parens() {
    use crate::nodes::{Begin, Gvar, Ivar, Lvar, Splat};

    let mut parser = RustParser::new(b"((*a), $x, @c)");
    assert_eq!(
        parser.parse_mlhs(),
        MLHS::DefinitelyMlhs {
            node: Box::new(Node::Begin(Begin {
                statements: vec![Node::Begin(Begin {
                    statements: vec![Node::Begin(Begin {
                        statements: vec![
                            Node::Begin(Begin {
                                statements: vec![Node::Begin(Begin {
                                    statements: vec![Node::Splat(Splat {
                                        value: Some(Box::new(Node::Lvar(Lvar {
                                            name: StringContent::from("a"),
                                            expression_l: loc!(3, 4)
                                        }))),
                                        operator_l: loc!(2, 3),
                                        expression_l: loc!(2, 4)
                                    })],
                                    begin_l: Some(loc!(2, 3)),
                                    end_l: Some(loc!(4, 5)),
                                    expression_l: loc!(2, 5)
                                })],
                                begin_l: Some(loc!(1, 2)),
                                end_l: Some(loc!(4, 5)),
                                expression_l: loc!(1, 5)
                            }),
                            Node::Gvar(Gvar {
                                name: StringContent::from("$x"),
                                expression_l: loc!(7, 9)
                            }),
                            Node::Ivar(Ivar {
                                name: StringContent::from("@c"),
                                expression_l: loc!(11, 13)
                            })
                        ],
                        begin_l: Some(loc!(1, 2)),
                        end_l: Some(loc!(13, 14)),
                        expression_l: loc!(1, 14)
                    })],
                    begin_l: Some(loc!(0, 1)),
                    end_l: Some(loc!(13, 14)),
                    expression_l: loc!(0, 14)
                })],
                begin_l: Some(loc!(0, 1)),
                end_l: Some(loc!(14, 15)),
                expression_l: loc!(0, 15)
            }))
        }
    );
}

#[test]
fn test_nameless_splat() {
    todo!("requires parse_primary");
}
