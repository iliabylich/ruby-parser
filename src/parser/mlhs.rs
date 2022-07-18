use crate::{
    builder::{Builder, Constructor},
    nodes::Node,
    parser::{result::ParserResultApi, ParseError, Parser},
    token::{Token, TokenKind},
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn parse_mlhs(&mut self) -> Result<MLHS, ParseError> {
        match parse_mlhs_internal(self)? {
            MlhsInternal::DefinitelyMlhsNode(node) => Ok(MLHS::DefinitelyMlhs { node }),
            MlhsInternal::DefinitelyMlhsList {
                nodes,
                trailing_comma,
            } => {
                let node = if let Some(trailing_comma) = trailing_comma {
                    Builder::<C>::group_with_trailing_comma(nodes, trailing_comma)
                } else {
                    Builder::<C>::group(nodes)
                };
                Ok(MLHS::DefinitelyMlhs { node })
            }
            MlhsInternal::MaybeLhs(node) => Ok(MLHS::MaybeLhs { node }),
            MlhsInternal::None => Err(ParseError::empty()),
        }
    }
}

fn parse_mlhs_internal<C: Constructor>(parser: &mut Parser<C>) -> Result<MlhsInternal, ParseError> {
    let mut items = vec![];
    let mut has_splat = false;
    let mut definitely_mlhs = false;
    let mut trailing_comma = None;

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

    loop {
        let item = parse_mlhs_item(parser).ignore_lookahead_errors()?;
        match item {
            Some(MlhsItem::DefinitelyMlhs(node)) => {
                definitely_mlhs = true;
                trailing_comma = None;

                handle_splat_argument!(&*node);

                items.push(*node);
            }

            Some(MlhsItem::MaybeLhs(node)) => {
                trailing_comma = None;

                items.push(*node);
            }

            Some(MlhsItem::None) | None => break,
        }

        if trailing_comma.is_none() && parser.current_token().is(TokenKind::tCOMMA) {
            // consume comma after MLHS item
            trailing_comma = Some(parser.take_token());
            definitely_mlhs = true;
        } else {
            break;
        }
    }

    if items.is_empty() {
        Ok(MlhsInternal::None)
    } else if definitely_mlhs {
        Ok(MlhsInternal::DefinitelyMlhsList {
            nodes: items,
            trailing_comma,
        })
    } else {
        debug_assert_eq!(items.len(), 1);

        let node = items.into_iter().next().unwrap();
        Ok(MlhsInternal::MaybeLhs(Box::new(node)))
    }
}

fn parse_mlhs_item<C: Constructor>(parser: &mut Parser<C>) -> Result<MlhsItem, ParseError> {
    parser
        .chain("mlhs item")
        .or_else(|| {
            let lparen_t = parser.try_token(TokenKind::tLPAREN)?;
            match parse_mlhs_internal(parser).ignore_lookahead_errors()? {
                Some(MlhsInternal::DefinitelyMlhsNode(inner)) => {
                    let rparen_t = parser.expect_token(TokenKind::tRPAREN);
                    let node = Builder::<C>::begin(lparen_t, vec![*inner], rparen_t);
                    Ok(MlhsItem::DefinitelyMlhs(node))
                }
                Some(MlhsInternal::DefinitelyMlhsList { nodes, .. }) => {
                    let rparen_t = parser.expect_token(TokenKind::tRPAREN);
                    let node = Builder::<C>::begin(lparen_t, nodes, rparen_t);
                    Ok(MlhsItem::DefinitelyMlhs(node))
                }
                Some(MlhsInternal::MaybeLhs(inner)) => {
                    let rparen_t = parser.expect_token(TokenKind::tRPAREN);
                    let node = Builder::<C>::begin(lparen_t, vec![*inner], rparen_t);
                    Ok(MlhsItem::MaybeLhs(node))
                }
                Some(MlhsInternal::None) | None => Ok(MlhsItem::None),
            }
        })
        .or_else(|| {
            let star_t = parser.try_token(TokenKind::tSTAR)?;
            match try_mlhs_primitive_item(parser).ignore_lookahead_errors()? {
                Some(node) => {
                    let node = Builder::<C>::splat(star_t, node);
                    Ok(MlhsItem::DefinitelyMlhs(node))
                }
                None => match parser.current_token().kind() {
                    TokenKind::tCOMMA | TokenKind::tRPAREN => {
                        let node = Builder::<C>::nameless_splat(star_t);
                        Ok(MlhsItem::DefinitelyMlhs(node))
                    }
                    _ => Ok(MlhsItem::None),
                },
            }
        })
        .or_else(|| {
            let a = match try_mlhs_primitive_item(parser).ignore_lookahead_errors()? {
                Some(node) => Ok(MlhsItem::MaybeLhs(node)),
                None => Ok(MlhsItem::None),
            };
            a
        })
        .done()
}

fn try_mlhs_primitive_item<C: Constructor>(
    parser: &mut Parser<C>,
) -> Result<Box<Node>, ParseError> {
    parser.try_lhs()
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum MLHS {
    // This variant is used if there's at least 1 comma
    // i.e. `a, b` or `((a, b))`
    // even `a,` is an MLHS
    DefinitelyMlhs { node: Box<Node> },

    // This variant is used if it looks like an LHS
    // i.e. has no commas but is techincally assignable
    // like `((a))`
    MaybeLhs { node: Box<Node> },
}

enum MlhsInternal {
    DefinitelyMlhsNode(Box<Node>),
    DefinitelyMlhsList {
        nodes: Vec<Node>,
        trailing_comma: Option<Token>,
    },
    MaybeLhs(Box<Node>),
    None,
}

#[derive(Debug)]
enum MlhsItem {
    DefinitelyMlhs(Box<Node>),
    MaybeLhs(Box<Node>),
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
        Ok(MLHS::MaybeLhs {
            node: Box::new(Node::Lvar(Lvar {
                name: StringContent::from("a"),
                expression_l: loc!(0, 1)
            }))
        })
    );
}

#[test]
fn test_lhs_parenthesized() {
    use crate::nodes::{Begin, Lvar};

    let mut parser = RustParser::new(b"((a))");
    assert_eq!(
        parser.parse_mlhs(),
        Ok(MLHS::MaybeLhs {
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
        })
    );
}

#[test]
fn test_mlhs_without_parens() {
    use crate::nodes::{Begin, Lvar, Splat};

    let mut parser = RustParser::new(b"a, *b, c");
    assert_eq!(
        parser.parse_mlhs(),
        Ok(MLHS::DefinitelyMlhs {
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
        })
    );
}

#[test]
fn test_mlhs_with_parens() {
    use crate::nodes::{Begin, Gvar, Ivar, Lvar, Splat};

    let mut parser = RustParser::new(b"((*a), $x, @c)");
    assert_eq!(
        parser.parse_mlhs(),
        Ok(MLHS::DefinitelyMlhs {
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
        })
    );
}

#[test]
fn test_nameless_splat() {
    let mut parser = RustParser::new(b"*");
    assert_eq!(parser.parse_mlhs(), Err(ParseError::empty()))
}
