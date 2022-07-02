use super::*;
use crate::builder::{Builder, Constructor};

impl<'a, C: Constructor> Parser<'a, C> {
    pub(crate) fn try_alias(&mut self) -> Option<Box<Node<'a>>> {
        let alias_t = self.try_token(TokenValue::kALIAS)?;

        let lhs;
        let rhs;

        if let Some(fitem) = self.try_fitem() {
            lhs = fitem;
            rhs = self
                .try_fitem()
                .unwrap_or_else(|| panic!("expected fitem, got {:?}", self.current_token()));
        } else {
            lhs = self
                .try_gvar()
                .unwrap_or_else(|| panic!("expected gvar, got {:?}", self.current_token()));
            rhs = None
                .or_else(|| self.try_gvar())
                .or_else(|| self.try_back_ref())
                .or_else(|| self.try_nth_ref())
                .unwrap_or_else(|| {
                    panic!(
                        "expected tGVAR/tBACK_REF/tNTH_REF, got {:?}",
                        self.current_token()
                    )
                });
        };

        Some(Builder::<C>::alias(alias_t, lhs, rhs))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        loc::loc,
        nodes::{Alias, Gvar, Sym},
        string_content::StringContent,
        Node, RustParser,
    };

    #[test]
    fn test_alias_name_to_name() {
        let mut parser = RustParser::new(b"alias foo bar");
        assert_eq!(
            parser.try_alias(),
            Some(Box::new(Node::Alias(Alias {
                to: Box::new(Node::Sym(Sym {
                    name: StringContent::from("foo"),
                    begin_l: None,
                    end_l: None,
                    expression_l: loc!(6, 9)
                })),
                from: Box::new(Node::Sym(Sym {
                    name: StringContent::from("bar"),
                    begin_l: None,
                    end_l: None,
                    expression_l: loc!(10, 13)
                })),
                keyword_l: loc!(0, 5),
                expression_l: loc!(0, 13)
            })))
        );
    }

    #[test]
    fn test_alias_sym_to_sym() {
        let mut parser = RustParser::new(b"alias :foo :bar");
        assert_eq!(
            parser.try_alias(),
            Some(Box::new(Node::Alias(Alias {
                to: Box::new(Node::Sym(Sym {
                    name: StringContent::from("foo"),
                    begin_l: Some(loc!(6, 7)),
                    end_l: None,
                    expression_l: loc!(6, 10)
                })),
                from: Box::new(Node::Sym(Sym {
                    name: StringContent::from("bar"),
                    begin_l: Some(loc!(11, 12)),
                    end_l: None,
                    expression_l: loc!(11, 15)
                })),
                keyword_l: loc!(0, 5),
                expression_l: loc!(0, 15)
            })))
        );
    }

    #[test]
    fn test_alias_gvar_to_gvar() {
        let mut parser = RustParser::new(b"alias $foo $bar");
        assert_eq!(
            parser.try_alias(),
            Some(Box::new(Node::Alias(Alias {
                to: Box::new(Node::Gvar(Gvar {
                    name: StringContent::from("$foo"),
                    expression_l: loc!(6, 10)
                })),
                from: Box::new(Node::Gvar(Gvar {
                    name: StringContent::from("$bar"),
                    expression_l: loc!(11, 15)
                })),
                keyword_l: loc!(0, 5),
                expression_l: loc!(0, 15)
            })))
        );
    }
}
