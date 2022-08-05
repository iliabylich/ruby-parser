use crate::{
    builder::Builder,
    nodes::Node,
    parser::{ParseResult, Parser},
    token::TokenKind,
};

impl Parser {
    pub(crate) fn parse_mlhs(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("mlhs")
            .or_else(|| {
                let exprs = self.parse_mlhs_basic()?;
                Ok(Builder::group(exprs))
            })
            .or_else(|| {
                let (lparen_t, exprs, rparen_t) = self
                    .all_of("( mlhs inner )")
                    .and(|| self.try_token(TokenKind::tLPAREN))
                    .and(|| self.parse_mlhs_inner())
                    .and(|| self.parse_rparen())
                    .stop()?;
                Ok(Builder::begin(lparen_t, exprs, rparen_t))
            })
            .compact()
            .stop()
    }

    fn parse_mlhs_inner(&mut self) -> ParseResult<Vec<Node>> {
        self.one_of("mlhs_basic")
            .or_else(|| self.parse_mlhs_basic())
            .or_else(|| {
                let (lparen_t, exprs, rparen_t) = self
                    .all_of("( mlhs inner )")
                    .and(|| self.try_token(TokenKind::tLPAREN))
                    .and(|| self.parse_mlhs_inner())
                    .and(|| self.parse_rparen())
                    .stop()?;

                let node = Builder::begin(lparen_t, exprs, rparen_t);

                Ok(vec![*node])
            })
            .stop()
    }

    fn parse_mlhs_basic(&mut self) -> ParseResult<Vec<Node>> {
        self.one_of("mlhs_basic")
            .or_else(|| {
                let (mut head, mut tail) = self
                    .all_of("mlhs head + tail")
                    .and(|| self.parse_mlhs_head().map(|mlhs_head| dbg!(mlhs_head)))
                    .and(|| {
                        self.one_of("mlhs head or tail")
                            .or_else(|| {
                                self.parse_mlhs_item().map(|node| {
                                    dbg!(&node);
                                    vec![*node]
                                })
                            })
                            .or_else(|| self.parse_mlhs_tail())
                            .or_else(|| Ok(vec![]))
                            .stop()
                    })
                    .stop()?;
                head.append(&mut tail);
                Ok(head)
            })
            .or_else(|| self.parse_mlhs_tail())
            .stop()
    }

    fn parse_mlhs_tail(&mut self) -> ParseResult<Vec<Node>> {
        let (star_t, maybe_splat_arg, mut post) = self
            .all_of("mlhs_tail")
            .and(|| self.try_token(TokenKind::tSTAR))
            .and(|| {
                self.one_of("splat argument")
                    .or_else(|| self.parse_mlhs_node().map(|node| Some(node)))
                    .or_else(|| Ok(None))
                    .stop()
            })
            .and(|| {
                self.one_of("post-splat")
                    .or_else(|| {
                        let (_comma_t, post) = self
                            .all_of("comma -> mlhs post")
                            .and(|| self.try_token(TokenKind::tCOMMA))
                            .and(|| self.parse_mlhs_post())
                            .stop()?;
                        Ok(post)
                    })
                    .or_else(|| Ok(vec![]))
                    .stop()
            })
            .stop()?;

        let splat = if let Some(value) = maybe_splat_arg {
            Builder::splat(star_t, value)
        } else {
            Builder::nameless_splat(star_t)
        };

        let mut items = Vec::with_capacity(1 + post.len());
        items.push(*splat);
        items.append(&mut post);
        Ok(items)
    }

    fn parse_mlhs_item(&mut self) -> ParseResult<Box<Node>> {
        let mlhs_item = self
            .one_of("mlhs item")
            .or_else(|| self.parse_mlhs_node())
            .or_else(|| {
                let (lparen_t, exprs, rparen_t) = self
                    .all_of("( mlhs inner )")
                    .and(|| self.try_token(TokenKind::tLPAREN))
                    .and(|| self.parse_mlhs_inner())
                    .and(|| self.parse_rparen())
                    .stop()?;
                Ok(Builder::begin(lparen_t, exprs, rparen_t))
            })
            .stop()?;
        dbg!(&mlhs_item);
        Ok(mlhs_item)
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

        dbg!(&head);
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
        let mlhs_node = self
            .one_of("mlhs node")
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
            .stop()?;

        if matches!(&*mlhs_node, Node::Ivar(_)) {
            // panic!("Created lost ivar")
        }
        dbg!(&mlhs_node);
        Ok(mlhs_node)
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
        assert_parses!(
            parse_mlhs,
            b"a, *b, c",
            r#"
s(:begin,
  s(:lvar, "a"),
  s(:splat,
    s(:lvar, "b")),
  s(:lvar, "c"))
            "#
        )
    }

    #[test]
    fn test_mlhs_with_parens() {
        assert_parses!(parse_mlhs, b"((*a), $x, @c)", "TODO")
    }

    #[test]
    fn test_nameless_splat() {
        assert_parses!(parse_mlhs, b"*", "s(:splat)")
    }
}
