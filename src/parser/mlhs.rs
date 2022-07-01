use crate::{
    builder::{Builder, Constructor},
    nodes::Node,
    parser::Parser,
    token::{Token, TokenValue},
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn parse_mlhs(&mut self) -> MLHS<'a> {
        match self.parse_mlhs2() {
            MlhsInternal::DefinitelyMlhsNode(node) => MLHS::DefinitelyMlhs { node },
            MlhsInternal::DefinitelyMlhsList {
                nodes,
                trailing_comma,
            } => {
                let node = if let Some(trailing_comma) = trailing_comma {
                    Builder::<C>::group_with_trailing_comma(nodes, trailing_comma)
                } else {
                    Builder::<C>::group(nodes)
                };
                MLHS::DefinitelyMlhs { node }
            }
            MlhsInternal::MaybeLhs(node) => MLHS::MaybeLhs { node },
            MlhsInternal::None => MLHS::None,
        }
    }

    fn parse_mlhs2(&mut self) -> MlhsInternal<'a> {
        let mut items = vec![];
        let mut has_splat = false;
        let mut definitely_mlhs = false;
        let mut has_trailing_comma = false;

        macro_rules! handle_splat_argument {
            ($node:expr) => {
                if matches!($node, Node::Splat(_)) {
                    if has_splat {
                        panic!("two splats in mlhs on the same level")
                    }
                    has_splat = true;
                }
            };
        }

        let is_splat = |node: &Node| matches!(node, Node::Splat(_));

        loop {
            match self.parse_mlhs_item() {
                MlhsInternal::DefinitelyMlhsNode(node) => {
                    definitely_mlhs = true;

                    handle_splat_argument!(&*node);
                    // if is_splat(&*node) {
                    //     if has_splat {
                    //         panic!("two splats in mlhs on the same level")
                    //     }
                    //     has_splat = true;
                    // }

                    items.push(*node);
                }

                MlhsInternal::DefinitelyMlhsList {
                    nodes,
                    trailing_comma,
                } => {
                    definitely_mlhs = true;

                    for node in nodes {
                        handle_splat_argument!(node);
                        items.push(node);
                    }
                }

                MlhsInternal::MaybeLhs(node) => {
                    items.push(*node);
                }

                MlhsInternal::None => break,
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

        let trailing_comma =
            if !has_trailing_comma && matches!(self.current_token().value(), TokenValue::tCOMMA) {
                // consume ,
                Some(self.take_token())
            } else {
                None
            };

        if items.is_empty() {
            MlhsInternal::None
        } else if definitely_mlhs {
            MlhsInternal::DefinitelyMlhsList {
                nodes: items,
                trailing_comma,
            }
        } else {
            debug_assert_eq!(items.len(), 1);

            let node = items.into_iter().next().unwrap();
            MlhsInternal::MaybeLhs(Box::new(node))
        }
    }

    fn parse_mlhs_item(&mut self) -> MlhsInternal<'a> {
        if let Some(lparen_t) = self.try_token(TokenValue::tLPAREN) {
            match self.parse_mlhs2() {
                MlhsInternal::DefinitelyMlhsNode(inner) => {
                    let rparen_t = self.expect_token(TokenValue::tRPAREN);
                    let node = Builder::<C>::begin(lparen_t, vec![*inner], rparen_t);
                    MlhsInternal::DefinitelyMlhsNode(node)
                }
                MlhsInternal::DefinitelyMlhsList { nodes, .. } => {
                    let rparen_t = self.expect_token(TokenValue::tRPAREN);
                    let node = Builder::<C>::begin(lparen_t, nodes, rparen_t);
                    MlhsInternal::DefinitelyMlhsNode(node)
                }
                MlhsInternal::MaybeLhs(inner) => {
                    let rparen_t = self.expect_token(TokenValue::tRPAREN);
                    let node = Builder::<C>::begin(lparen_t, vec![*inner], rparen_t);
                    MlhsInternal::MaybeLhs(node)
                }
                MlhsInternal::None => MlhsInternal::None,
            }
        } else if let Some(star_t) = self.try_token(TokenValue::tSTAR) {
            match self.parse_mlhs_primitive_item() {
                Some(node) => {
                    let node = Builder::<C>::splat(star_t, node);
                    MlhsInternal::DefinitelyMlhsNode(node)
                }
                None => match self.current_token().value() {
                    TokenValue::tCOMMA | TokenValue::tRPAREN => {
                        let node = Builder::<C>::nameless_splat(star_t);
                        MlhsInternal::DefinitelyMlhsNode(node)
                    }
                    _ => MlhsInternal::None,
                },
            }
        } else {
            match self.parse_mlhs_primitive_item() {
                Some(node) => MlhsInternal::MaybeLhs(node),
                None => MlhsInternal::None,
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

enum MlhsInternal<'a> {
    DefinitelyMlhsNode(Box<Node<'a>>),
    DefinitelyMlhsList {
        nodes: Vec<Node<'a>>,
        trailing_comma: Option<Token<'a>>,
    },
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
                statements: vec![
                    Node::Begin(Begin {
                        statements: vec![Node::Splat(Splat {
                            value: Some(Box::new(Node::Lvar(Lvar {
                                name: StringContent::from("a"),
                                expression_l: loc!(3, 4)
                            }))),
                            operator_l: loc!(2, 3),
                            expression_l: loc!(2, 4)
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
                begin_l: Some(loc!(0, 1)),
                end_l: Some(loc!(13, 14)),
                expression_l: loc!(0, 14)
            }))
        }
    );
}

#[test]
fn test_nameless_splat() {
    todo!("requires parse_primary");
}
