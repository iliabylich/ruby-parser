use crate::{
    nodes::Node,
    parser::{ParseResult, Parser},
    token::TokenKind,
};

impl Parser {
    pub(crate) fn parse_mlhs(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("mlhs")
            .or_else(|| self.parse_mlhs_basic())
            .or_else(|| {
                self.all_of("( mlhs inner )")
                    .and(|| self.try_token(TokenKind::tLPAREN))
                    .and(|| self.parse_mlhs_inner())
                    .and(|| self.parse_rparen())
                    .stop()
                    .map(|(lparen_t, exprs, rparen_t)| {
                        todo!("{:?} {:?} {:?}", lparen_t, exprs, rparen_t)
                    })
            })
            .stop()
    }

    fn parse_mlhs_inner(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("mlhs_basic")
            .or_else(|| self.parse_mlhs_basic())
            .or_else(|| {
                self.all_of("( mlhs inner )")
                    .and(|| self.try_token(TokenKind::tLPAREN))
                    .and(|| self.parse_mlhs_inner())
                    .and(|| self.parse_rparen())
                    .stop()
                    .map(|(lparen_t, exprs, rparen_t)| {
                        todo!("{:?} {:?} {:?}", lparen_t, exprs, rparen_t)
                    })
            })
            .stop()
    }

    fn parse_mlhs_basic(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("mlhs_basic")
            .or_else(|| {
                self.all_of("mlhs head + tail")
                    .and(|| self.parse_mlhs_head())
                    .and(|| {
                        self.one_of("mlhs head or tail")
                            .or_else(|| self.parse_mlhs_item().map(|node| Some(node)))
                            .or_else(|| self.parse_mlhs_tail().map(|node| Some(node)))
                            .or_else(|| Ok(None))
                            .stop()
                    })
                    .stop()
                    .map(|(head, tail)| panic!("{:?} {:?}", head, tail))
            })
            .or_else(|| self.parse_mlhs_tail())
            .stop()
    }

    fn parse_mlhs_tail(&mut self) -> ParseResult<Box<Node>> {
        let (star_t, maybe_splat_arg, maybe_post) = self
            .all_of("mlhs_tail")
            .and(|| self.try_token(TokenKind::tSTAR))
            .and(|| {
                self.one_of("splat argument")
                    .or_else(|| self.parse_mlhs_node().map(|node| Some(node)))
                    .or_else(|| Ok(None))
                    .stop()
            })
            .and(|| {
                let maybe_splat: Option<Box<Node>> = self
                    .one_of("post-splat")
                    .or_else(|| {
                        self.all_of("comma -> mlhs post")
                            .and(|| self.try_token(TokenKind::tCOMMA))
                            .and(|| self.parse_mlhs_post())
                            .stop()
                            .map(|(star_t, maybe_arg)| todo!("{:?} {:?}", star_t, maybe_arg))
                    })
                    .or_else(|| Ok(None))
                    .stop()?;

                Ok(maybe_splat)
            })
            .stop()?;

        todo!("{:?} {:?} {:?}", star_t, maybe_splat_arg, maybe_post)
    }

    fn parse_mlhs_item(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("mlhs item")
            .or_else(|| self.parse_mlhs_node())
            .or_else(|| {
                let (lparen_t, exprs, rparen_t) = self
                    .all_of("( mlhs inner )")
                    .and(|| self.try_token(TokenKind::tLPAREN))
                    .and(|| self.parse_mlhs_inner())
                    .and(|| self.parse_rparen())
                    .stop()?;
                todo!("{:?} {:?} {:?}", lparen_t, exprs, rparen_t)
            })
            .stop()
    }

    fn parse_mlhs_head(&mut self) -> ParseResult<Vec<Node>> {
        let parse_item_and_comma = |parser: &mut Parser| {
            parser
                .all_of("mlhs item and comma")
                .and(|| parser.parse_mlhs_item())
                .and(|| parser.try_token(TokenKind::tCOMMA))
                .stop()
        };

        let mut head = vec![];
        let mut commas = vec![];
        let (item, comma) = parse_item_and_comma(self)?;
        head.push(*item);
        commas.push(comma);

        loop {
            let item_and_comma = parse_item_and_comma(self);

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

    fn parse_mlhs_post(&mut self) -> ParseResult<Vec<Node>> {
        let mut post = vec![];
        let mut commas = vec![];
        let item = self.parse_mlhs_item()?;
        post.push(*item);

        loop {
            let comma_and_item = self
                .all_of("comma and mlhs item")
                .and(|| self.try_token(TokenKind::tCOMMA))
                .and(|| self.parse_mlhs_item())
                .stop();

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

    fn parse_mlhs_node(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("mlhs node")
            .or_else(|| self.parse_user_variable())
            .or_else(|| self.parse_keyword_variable())
            .or_else(|| self.parse_back_ref())
            .or_else(|| {
                let (colon2_t, const_t) = self.parse_colon2_const()?;
                panic!("const {:?} {:?}", colon2_t, const_t)
            })
            .or_else(|| {
                let (primary_value, op_t, id_t) = self
                    .all_of("primary call_op [const/tIDENT]")
                    .and(|| self.parse_primary_value())
                    .and(|| self.parse_call_op())
                    .and(|| self.parse_const_or_identifier())
                    .stop()?;

                panic!(
                    "primary_value call_op tIDENT {:?} {:?} {:?}",
                    primary_value, op_t, id_t
                )
            })
            .or_else(|| {
                let (primary_value, colon2_t, const_t) = self
                    .all_of("priamay :: [const/tIDENT")
                    .and(|| self.parse_primary_value())
                    .and(|| self.expect_token(TokenKind::tCOLON2))
                    .and(|| self.parse_const_or_identifier())
                    .stop()?;

                panic!(
                    "primary_value tCOLON2 tCONSTANT {:?} {:?} {:?}",
                    primary_value, colon2_t, const_t
                )
            })
            .stop()
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_lhs_user_variable() {
        assert_parses!(parse_mlhs, b"a, b", "TODO")
    }

    #[test]
    fn test_lhs_parenthesized() {
        assert_parses!(parse_mlhs, b"((a))", "TODO")
    }

    #[test]
    fn test_mlhs_without_parens() {
        assert_parses!(parse_mlhs, b"a, *b, c", "TODO")
    }

    #[test]
    fn test_mlhs_with_parens() {
        assert_parses!(parse_mlhs, b"((*a), $x, @c)", "TODO")
    }

    #[test]
    fn test_nameless_splat() {
        assert_parses!(parse_mlhs, b"*", "TODO")
    }
}
