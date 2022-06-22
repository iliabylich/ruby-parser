use super::*;
use crate::builder::alias;

impl<'a> Parser<'a> {
    pub(crate) fn parse_alias(&mut self) -> Option<Box<Node<'a>>> {
        if self.current_token().value() != &TokenValue::kALIAS {
            return None;
        }

        let alias_t = self.take_token();

        let lhs;
        let rhs;

        if let Some(fitem) = self.parse_fitem() {
            lhs = fitem;
            rhs = self
                .parse_fitem()
                .unwrap_or_else(|| panic!("expected fitem, got {:?}", self.current_token()));
        } else {
            lhs = self
                .parse_gvar()
                .unwrap_or_else(|| panic!("expected gvar, got {:?}", self.current_token()));
            rhs = None
                .or_else(|| self.parse_gvar())
                .or_else(|| self.parse_back_ref())
                .or_else(|| self.parse_nth_ref())
                .unwrap_or_else(|| {
                    panic!(
                        "expected tGVAR/tBACK_REF/tNTH_REF, got {:?}",
                        self.current_token()
                    )
                });
        };

        Some(alias(alias_t, lhs, rhs))
    }
}

#[test]
fn test_alias_fitem_fitem() {
    let mut parser = Parser::new(b"alias foo bar");
    panic!("{:?}", parser.parse_alias());
}

#[test]
fn test_alias_gvar_gvar() {
    use crate::{
        nodes::{Alias, Gvar},
        Loc, Node,
    };

    let mut parser = Parser::new(b"alias $foo $bar");
    assert_eq!(
        parser.parse_alias(),
        Some(Box::new(Node::Alias(Alias {
            to: Box::new(Node::Gvar(Gvar {
                name: String::from("foo"),
                expression_l: Loc(6, 10)
            })),
            from: Box::new(Node::Gvar(Gvar {
                name: String::from("bar"),
                expression_l: Loc(11, 15)
            })),
            keyword_l: Loc(0, 5),
            expression_l: Loc(0, 15)
        })))
    );
}
