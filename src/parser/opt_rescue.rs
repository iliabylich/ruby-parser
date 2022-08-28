use crate::{
    builder::Builder,
    parser::{
        macros::{all_of, at_least_once, maybe, one_of},
        ParseResult, Parser,
    },
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    // This rule can be `none`
    pub(crate) fn parse_opt_rescue(&mut self) -> ParseResult<Vec<Node>> {
        let nodes = maybe!(at_least_once!("opt_rescue", parse_opt_rescue1(self)))?;
        Ok(nodes.unwrap_or_else(|| vec![]))
    }

    pub(crate) fn parse_then(&mut self) -> ParseResult<Token> {
        one_of!(
            "then ...",
            checkpoint = self.new_checkpoint(),
            self.parse_term(),
            self.try_token(TokenKind::kTHEN),
            {
                let (_term, then_t) = all_of!(
                    "term then",
                    self.parse_term(),
                    self.try_token(TokenKind::kTHEN),
                )?;
                Ok(then_t)
            },
        )
    }

    pub(crate) fn parse_lhs(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "lhs",
            checkpoint = self.new_checkpoint(),
            self.parse_user_variable(),
            self.parse_keyword_variable(),
            self.parse_back_ref(),
            {
                let (colon2_t, name_t) = self.parse_colon2_const()?;
                Ok(Builder::const_global(colon2_t, name_t, self.buffer()))
            },
            {
                let (primary_value, op_t, id_t) = all_of!(
                    "primary call_op [const/tIDENT]",
                    self.parse_primary_value(),
                    self.parse_call_op2(),
                    self.parse_const_or_identifier(),
                )?;

                panic!(
                    "primary_value call_op tIDENT {:?} {:?} {:?}",
                    primary_value, op_t, id_t
                )
            },
        )
    }

    pub(crate) fn parse_arg_value(&mut self) -> ParseResult<Box<Node>> {
        self.parse_arg()
    }
}

fn parse_opt_rescue1(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (rescue_t, exc_list, exc_var, then, compstmt) = all_of!(
        "rescue1",
        parser.try_token(TokenKind::kRESCUE),
        parse_exc_list(parser),
        try_exc_var(parser),
        {
            one_of!(
                "optional then",
                checkpoint = parser.new_checkpoint(),
                parser.parse_then().map(|tok| Some(tok)),
                Ok(None),
            )
        },
        parser.try_compstmt(),
    )?;

    Ok(Builder::rescue_body(
        rescue_t, exc_list, exc_var, then, compstmt,
    ))
}

fn parse_exc_list(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    one_of!(
        "exceptions list",
        checkpoint = parser.new_checkpoint(),
        parser.parse_mrhs(),
        parser.parse_arg_value().map(|arg_value| vec![*arg_value]),
        Ok(vec![]),
    )
}
fn try_exc_var(parser: &mut Parser) -> ParseResult<Option<(Token, Box<Node>)>> {
    one_of!(
        "[exc var]",
        checkpoint = parser.new_checkpoint(),
        {
            let (assoc_t, lhs) = all_of!(
                "exc var",
                parser.try_token(TokenKind::tASSOC),
                parser.parse_lhs(),
            )?;
            Ok(Some((assoc_t, lhs)))
        },
        Ok(None),
    )
}

#[cfg(test)]
mod tests {
    use super::parse_opt_rescue1;
    use crate::testing::assert_parses;

    #[test]
    fn test_opt_rescue1_simple() {
        assert_parses!(parse_opt_rescue1, b"rescue", "s(:resbody, nil, nil, nil)");
    }

    #[test]
    fn test_opt_rescue1_full() {
        assert_parses!(
            parse_opt_rescue1,
            b"rescue Foo, Bar::Baz => e; 42",
            r#"
s(:resbody,
  s(:array,
    s(:const, nil, "Foo"),
    s(:const,
      s(:const, nil, "Bar"), "Baz")),
  s(:lvar, "e"),
  s(:int, "42"))
            "#
        );
    }
}
