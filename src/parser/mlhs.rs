use crate::{
    builder::Constructor,
    nodes::Node,
    parser::{ParseResult, Parser},
    token::TokenKind,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_mlhs(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("mlhs")
            .or_else(|| self.try_mlhs_basic())
            .or_else(|| {
                self.all_of("( mlhs inner )")
                    .and(|| self.try_token(TokenKind::tLPAREN))
                    .and(|| self.try_mlhs_inner())
                    .and(|| self.try_rparen())
                    .unwrap()
                    .map(|(lparen_t, exprs, rparen_t)| {
                        todo!("{:?} {:?} {:?}", lparen_t, exprs, rparen_t)
                    })
            })
            .unwrap()
    }

    fn try_mlhs_inner(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("mlhs_basic")
            .or_else(|| self.try_mlhs_basic())
            .or_else(|| {
                self.all_of("( mlhs inner )")
                    .and(|| self.try_token(TokenKind::tLPAREN))
                    .and(|| self.try_mlhs_inner())
                    .and(|| self.try_rparen())
                    .unwrap()
                    .map(|(lparen_t, exprs, rparen_t)| {
                        todo!("{:?} {:?} {:?}", lparen_t, exprs, rparen_t)
                    })
            })
            .unwrap()
    }

    fn try_mlhs_basic(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("mlhs_basic")
            .or_else(|| {
                self.all_of("mlhs head + tail")
                    .and(|| self.try_mlhs_head())
                    .and(|| {
                        self.one_of("mlhs head or tail")
                            .or_else(|| self.try_mlhs_item().map(|node| Some(node)))
                            .or_else(|| self.try_mlhs_tail().map(|node| Some(node)))
                            .or_else(|| Ok(None))
                            .unwrap()
                    })
                    .unwrap()
                    .map(|(head, tail)| panic!("{:?} {:?}", head, tail))
            })
            .or_else(|| self.try_mlhs_tail())
            .unwrap()
    }

    fn try_mlhs_tail(&mut self) -> ParseResult<Box<Node>> {
        let (star_t, maybe_splat_arg, maybe_post) = self
            .all_of("mlhs_tail")
            .and(|| self.try_token(TokenKind::tSTAR))
            .and(|| {
                self.one_of("splat argument")
                    .or_else(|| self.try_mlhs_node().map(|node| Some(node)))
                    .or_else(|| Ok(None))
                    .unwrap()
            })
            .and(|| {
                let maybe_splat: Option<Box<Node>> = self
                    .one_of("post-splat")
                    .or_else(|| {
                        self.all_of("comma -> mlhs post")
                            .and(|| self.try_token(TokenKind::tCOMMA))
                            .and(|| self.try_mlhs_post())
                            .unwrap()
                            .map(|(star_t, maybe_arg)| todo!("{:?} {:?}", star_t, maybe_arg))
                    })
                    .or_else(|| Ok(None))
                    .unwrap()?;

                Ok(maybe_splat)
            })
            .unwrap()?;

        todo!("{:?} {:?} {:?}", star_t, maybe_splat_arg, maybe_post)
    }

    fn try_mlhs_item(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("mlhs item")
            .or_else(|| self.try_mlhs_node())
            .or_else(|| {
                let (lparen_t, exprs, rparen_t) = self
                    .all_of("( mlhs inner )")
                    .and(|| self.try_token(TokenKind::tLPAREN))
                    .and(|| self.try_mlhs_inner())
                    .and(|| self.try_rparen())
                    .unwrap()?;
                todo!("{:?} {:?} {:?}", lparen_t, exprs, rparen_t)
            })
            .unwrap()
    }

    fn try_mlhs_head(&mut self) -> ParseResult<Vec<Node>> {
        let try_item_and_comma = |parser: &mut Parser<C>| {
            parser
                .all_of("mlhs item and comma")
                .and(|| parser.try_mlhs_item())
                .and(|| parser.try_token(TokenKind::tCOMMA))
                .unwrap()
        };

        let mut head = vec![];
        let mut commas = vec![];
        let (item, comma) = try_item_and_comma(self)?;
        head.push(*item);
        commas.push(comma);

        loop {
            let item_and_comma = try_item_and_comma(self);

            match item_and_comma {
                Ok((item, comma)) => {
                    head.push(*item);
                    commas.push(comma);
                }
                Err(_) => break,
            }
        }

        Ok(head)
    }

    fn try_mlhs_post(&mut self) -> ParseResult<Vec<Node>> {
        let mut post = vec![];
        let mut commas = vec![];
        let item = self.try_mlhs_item()?;
        post.push(*item);

        loop {
            let comma_and_item = self
                .all_of("comma and mlhs item")
                .and(|| self.try_token(TokenKind::tCOMMA))
                .and(|| self.try_mlhs_item())
                .unwrap();

            match comma_and_item {
                Ok((comma, item)) => {
                    post.push(*item);
                    commas.push(comma);
                }
                Err(_) => break,
            }
        }

        Ok(post)
    }

    fn try_mlhs_node(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("mlhs node")
            .or_else(|| self.try_user_variable())
            .or_else(|| self.try_keyword_variable())
            .or_else(|| self.try_back_ref())
            .or_else(|| {
                let (colon2_t, const_t) = self.try_colon2_const()?;
                panic!("const {:?} {:?}", colon2_t, const_t)
            })
            .or_else(|| {
                let (primary_value, op_t, id_t) = self
                    .all_of("primary call_op [const/tIDENT]")
                    .and(|| self.try_primary_value())
                    .and(|| self.try_call_op())
                    .and(|| self.try_const_or_identifier())
                    .unwrap()?;

                panic!(
                    "primary_value call_op tIDENT {:?} {:?} {:?}",
                    primary_value, op_t, id_t
                )
            })
            .or_else(|| {
                let (primary_value, colon2_t, const_t) = self
                    .all_of("priamay :: [const/tIDENT")
                    .and(|| self.try_primary_value())
                    .and(|| self.expect_token(TokenKind::tCOLON2))
                    .and(|| self.try_const_or_identifier())
                    .unwrap()?;

                panic!(
                    "primary_value tCOLON2 tCONSTANT {:?} {:?} {:?}",
                    primary_value, colon2_t, const_t
                )
            })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{ParseError, RustParser};

    #[test]
    fn test_lhs_user_variable() {
        let mut parser = RustParser::new(b"a, b");
        assert_eq!(parser.try_mlhs(), Err(ParseError::empty()));
    }

    #[test]
    fn test_lhs_parenthesized() {
        let mut parser = RustParser::new(b"((a))");
        assert_eq!(parser.try_mlhs(), Err(ParseError::empty()));
    }

    #[test]
    fn test_mlhs_without_parens() {
        let mut parser = RustParser::new(b"a, *b, c");
        assert_eq!(parser.try_mlhs(), Err(ParseError::empty()));
    }

    #[test]
    fn test_mlhs_with_parens() {
        let mut parser = RustParser::new(b"((*a), $x, @c)");
        assert_eq!(parser.try_mlhs(), Err(ParseError::empty()));
    }

    #[test]
    fn test_nameless_splat() {
        let mut parser = RustParser::new(b"*");
        assert_eq!(parser.try_mlhs(), Err(ParseError::empty()))
    }
}
