use crate::{
    builder::Builder,
    nodes::{Begin, Node},
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
};

/*
        mlhs: ( mlhs )
            | mlhs_head mlhs_tail

   mlhs_head: node
            | tSTAR node

   mlhs_tail: none
            | tCOMMA mlhs
            | tCOMMA mlhs mlhs_tail

        node: user_variable
            | keyword_variable
            | primary_value tLBRACK2 opt_call_args rbracket
            | primary_value call_op2 tIDENTIFIER
            | primary_value call_op2 tCONSTANT
            | tCOLON3 tCONSTANT
            | backref
*/
impl Parser {
    pub(crate) fn parse_mlhs(&mut self) -> ParseResult<Box<Node>> {
        if self.current_token().is(TokenKind::tLPAREN) {
            let (lparen_t, mut inner, rparen_t) = self
                .all_of("( mlhs )")
                .and(|| self.try_token(TokenKind::tLPAREN))
                .and(|| parse_mlhs_list(self))
                .and(|| self.expect_token(TokenKind::tRPAREN))
                .stop()?;

            if inner.len() == 1 {
                if let Node::Begin(Begin {
                    statements,
                    begin_l,
                    end_l,
                    ..
                }) = &mut inner[0]
                {
                    if statements.len() == 1 && begin_l.is_none() && end_l.is_none() {
                        // collapse `inner`
                        inner = vec![std::mem::take(statements).into_iter().next().unwrap()]
                    }
                }
            }

            Ok(Builder::begin(lparen_t, inner, rparen_t))
        } else {
            let (head, mut tail) = self
                .all_of("mlhs head + mlhs tail")
                .and(|| parse_mlhs_head(self))
                .and(|| parse_mlhs_tail(self))
                .stop()?;

            if tail.is_empty() {
                Ok(head)
            } else {
                let mut nodes = Vec::with_capacity(1 + tail.len());
                nodes.push(*head);
                nodes.append(&mut tail);
                Ok(Builder::group(nodes))
            }
        }
    }
}

fn parse_mlhs_list(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    let mut items = vec![];
    let mut commas = vec![];

    fn append_mlhs(parser: &mut Parser, items: &mut Vec<Node>) -> ParseResult<()> {
        let mlhs = parser.parse_mlhs()?;

        match *mlhs {
            Node::Begin(Begin {
                mut statements,
                begin_l,
                end_l,
                ..
            }) if begin_l.is_none() && end_l.is_none() => items.append(&mut statements),
            single => {
                items.push(single);
            }
        }

        Ok(())
    }

    append_mlhs(parser, &mut items)?;

    loop {
        if parser.current_token().is(TokenKind::tCOMMA) {
            commas.push(parser.current_token());
            parser.skip_token();
        } else {
            break;
        }

        match append_mlhs(parser, &mut items) {
            Ok(_) => continue,
            Err(_) => break,
        }
    }

    Ok(items)
}

fn parse_mlhs_head(parser: &mut Parser) -> ParseResult<Box<Node>> {
    if parser.current_token().is(TokenKind::tSTAR) {
        let star_t = parser.current_token();
        parser.skip_token();

        if let Ok(value) = parse_mlhs_node(parser) {
            Ok(Builder::splat(star_t, value))
        } else {
            Ok(Builder::nameless_splat(star_t))
        }
    } else {
        parse_mlhs_node(parser)
    }
}

fn parse_mlhs_tail(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    let mut commas = vec![];
    let mut items = vec![];

    fn parse_comma_and_item(parser: &mut Parser) -> Option<(Token, Box<Node>)> {
        let checkpoint = parser.new_checkpoint();
        if parser.current_token().is(TokenKind::tCOMMA) {
            let comma = parser.current_token();
            parser.skip_token();

            match parser.parse_mlhs() {
                Ok(item) => Some((comma, item)),
                Err(_) => {
                    checkpoint.restore();
                    None
                }
            }
        } else {
            None
        }
    }

    loop {
        match parse_comma_and_item(parser) {
            Some((comma, item)) => {
                commas.push(comma);

                match *item {
                    Node::Begin(Begin {
                        mut statements,
                        begin_l,
                        end_l,
                        ..
                    }) if begin_l.is_none() && end_l.is_none() => items.append(&mut statements),
                    single => {
                        items.push(single);
                    }
                }
            }
            None => break,
        }
    }

    Ok(items)
}

fn parse_mlhs_node(parser: &mut Parser) -> ParseResult<Box<Node>> {
    parser
        .one_of("mlhs node")
        .or_else(|| parser.parse_user_variable())
        .or_else(|| parser.parse_keyword_variable())
        .or_else(|| parser.parse_back_ref())
        .or_else(|| {
            let (colon2_t, name_t) = parser.parse_colon2_const()?;
            Ok(Builder::const_global(colon2_t, name_t, parser.buffer()))
        })
        .or_else(|| {
            let (primary_value, op_t, id_t) = parser
                .all_of("primary call_op [const/tIDENT]")
                .and(|| parser.parse_primary_value())
                .and(|| parser.parse_call_op())
                .and(|| parser.parse_const_or_identifier())
                .stop()?;

            panic!(
                "primary_value call_op tIDENT {:?} {:?} {:?}",
                primary_value, op_t, id_t
            )
        })
        .or_else(|| {
            let (primary_value, colon2_t, const_t) = parser
                .all_of("priamay :: [const/tIDENT")
                .and(|| parser.parse_primary_value())
                .and(|| parser.expect_token(TokenKind::tCOLON2))
                .and(|| parser.parse_const_or_identifier())
                .stop()?;

            panic!(
                "primary_value tCOLON2 tCONSTANT {:?} {:?} {:?}",
                primary_value, colon2_t, const_t
            )
        })
        .stop()
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_lhs_user_variable() {
        assert_parses!(
            parse_mlhs,
            b"a, b",
            r#"
s(:begin,
  s(:lvar, "a"),
  s(:lvar, "b"))
            "#
        )
    }

    #[test]
    fn test_lhs_parenthesized() {
        assert_parses!(
            parse_mlhs,
            b"((a))",
            r#"
s(:begin,
  s(:begin,
    s(:lvar, "a")))
            "#
        )
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
        assert_parses!(
            parse_mlhs,
            b"((*a), @b, $c)",
            r#"
s(:begin,
  s(:begin,
    s(:splat,
      s(:lvar, "a"))),
  s(:ivar, "@b"),
  s(:gvar, "$c"))
            "#
        );
    }

    #[test]
    fn test_nameless_splat() {
        assert_parses!(parse_mlhs, b"*", "s(:splat)");
    }
}
