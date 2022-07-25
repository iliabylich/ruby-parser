use crate::{
    builder::{Builder, Constructor},
    parser::{ParseError, Parser},
    token::TokenKind,
    Node,
};

impl<C: Constructor> Parser<C> {
    pub(crate) fn try_alias(&mut self) -> Result<Box<Node>, ParseError> {
        let (alias_t, (lhs, rhs)) = self
            .all_of("alias statement")
            .and(|| self.try_token(TokenKind::kALIAS))
            .and(|| parse_alias_args(self))
            .unwrap()?;
        Ok(Builder::<C>::alias(alias_t, lhs, rhs))
    }
}

fn parse_alias_args<C: Constructor>(
    parser: &mut Parser<C>,
) -> Result<(Box<Node>, Box<Node>), ParseError> {
    parser
        .one_of("alias arguments")
        .or_else(|| try_fitem_fitem(parser))
        .or_else(|| try_gvar_gvar(parser))
        .required()
        .compact()
        .unwrap()
}

fn try_fitem_fitem<C: Constructor>(
    parser: &mut Parser<C>,
) -> Result<(Box<Node>, Box<Node>), ParseError> {
    parser
        .all_of("fitem -> fitem")
        .and(|| parser.try_fitem())
        .and(|| parser.try_fitem())
        .unwrap()
}

fn try_gvar_gvar<C: Constructor>(
    parser: &mut Parser<C>,
) -> Result<(Box<Node>, Box<Node>), ParseError> {
    parser
        .all_of("gvar -> [gvar | back ref | nth ref]")
        .and(|| parser.try_gvar())
        .and(|| {
            parser
                .one_of("gvar rhs")
                .or_else(|| parser.try_gvar())
                .or_else(|| parser.try_back_ref())
                .or_else(|| parser.try_nth_ref())
                .required()
                .unwrap()
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{
        loc::loc,
        nodes::{Alias, Gvar, Sym},
        string_content::StringContent,
        transactions::assert_err_eq,
        Node, RustParser,
    };

    #[test]
    fn test_alias_name_to_name() {
        let mut parser = RustParser::new(b"alias foo bar");
        assert_eq!(
            parser.try_alias(),
            Ok(Box::new(Node::Alias(Alias {
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
            Ok(Box::new(Node::Alias(Alias {
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
            Ok(Box::new(Node::Alias(Alias {
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

    #[test]
    fn test_nothing() {
        let mut parser = RustParser::new(b"");
        assert_err_eq!(
            parser.try_alias(),
            "
SEQUENCE (1) alias statement (got [])
    TOKEN (1) expected kALIAS, got tEOF (at 0)
"
        );
    }

    #[test]
    fn test_only_alias() {
        let mut parser = RustParser::new(b"alias $foo");
        assert_err_eq!(
            parser.try_alias(),
            "
SEQUENCE (11) alias statement (got [Token(Token { kind: kALIAS, loc: 0...5, value: None })])
    ONEOF (11) alias arguments
        SEQUENCE (11) gvar -> [gvar | back ref | nth ref] (got [Node(Gvar(Gvar { name: StringContent { bytes: [36, 102, 111, 111] }, expression_l: 6...10 }))])
            ONEOF (1) gvar rhs
                TOKEN (1) expected tGVAR, got tEOF (at 10)
                TOKEN (1) expected tBACK_REF, got tEOF (at 10)
                TOKEN (1) expected tNTH_REF, got tEOF (at 10)
"
        );
    }
}
