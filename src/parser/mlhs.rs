use crate::{
    builder::Builder,
    nodes::{Begin, Node},
    parser::{
        macros::{all_of, at_least_once, maybe, one_of, separated_by},
        ParseResult, Parser,
    },
    token::TokenKind,
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
            let (lparen_t, mut inner, rparen_t) = all_of!(
                "( mlhs )",
                self.try_token(TokenKind::tLPAREN),
                parse_mlhs_list(self),
                self.expect_token(TokenKind::tRPAREN),
            )?;

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
            let (head, mut tail) = all_of!(
                "mlhs head + mlhs tail",
                parse_mlhs_head(self),
                parse_mlhs_tail(self),
            )?;

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
    let items = maybe!(separated_by!(
        "mlhs list",
        checkpoint = parser.new_checkpoint(),
        item = parser.parse_mlhs(),
        sep = parser.try_token(TokenKind::tCOMMA)
    ))?;

    match items {
        Some((items, _commas)) => Ok(mlhs_list_flatten(items)),
        None => Ok(vec![]),
    }
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
    let items = maybe!(at_least_once!(
        "mlhs tail",
        all_of!(
            "mlhs tail item",
            parser.try_token(TokenKind::tCOMMA),
            parser.parse_mlhs(),
        )
        .map(|(_comma, item)| item)
    ))?;

    match items {
        Some(items) => Ok(mlhs_list_flatten(items)),
        None => Ok(vec![]),
    }
}

fn parse_mlhs_node(parser: &mut Parser) -> ParseResult<Box<Node>> {
    one_of!(
        "mlhs node",
        checkpoint = parser.new_checkpoint(),
        parser.parse_user_variable(),
        parser.parse_keyword_variable(),
        parser.parse_back_ref(),
        {
            let (colon2_t, name_t) = parser.parse_colon2_const()?;
            Ok(Builder::const_global(colon2_t, name_t, parser.buffer()))
        },
        {
            let (primary_value, op_t, id_t) = all_of!(
                "primary call_op [const/tIDENT]",
                parser.parse_primary_value(),
                parser.parse_call_op(),
                parser.parse_const_or_identifier(),
            )?;

            panic!(
                "primary_value call_op tIDENT {:?} {:?} {:?}",
                primary_value, op_t, id_t
            )
        },
        {
            let (primary_value, colon2_t, const_t) = all_of!(
                "priamay :: [const/tIDENT",
                parser.parse_primary_value(),
                parser.expect_token(TokenKind::tCOLON2),
                parser.parse_const_or_identifier(),
            )?;

            panic!(
                "primary_value tCOLON2 tCONSTANT {:?} {:?} {:?}",
                primary_value, colon2_t, const_t
            )
        },
    )
}

fn mlhs_list_flatten(items: Vec<Node>) -> Vec<Node> {
    let mut flatten = vec![];
    for item in items.into_iter() {
        match item {
            Node::Begin(Begin {
                mut statements,
                begin_l,
                end_l,
                ..
            }) if begin_l.is_none() && end_l.is_none() => flatten.append(&mut statements),
            single => {
                flatten.push(single);
            }
        }
    }
    flatten
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
