use crate::{
    builder::Builder,
    parser::{
        macros::{all_of, one_of},
        ParseError, ParseResult, Parser,
    },
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    // This rule can be `none`
    pub(crate) fn parse_opt_rescue(&mut self) -> ParseResult<Vec<Node>> {
        let mut nodes = vec![];
        loop {
            match parse_opt_rescue1(self) {
                Ok(node) => nodes.push(*node),
                Err(error) => {
                    match error.strip_lookaheads() {
                        None => {
                            // no match
                            break;
                        }
                        Some(error) => {
                            return Err(ParseError::seq_error("opt rescue", nodes, error));
                        }
                    }
                }
            }
        }
        Ok(nodes)
    }

    pub(crate) fn parse_then(&mut self) -> ParseResult<Token> {
        one_of!(
            "then ...",
            checkpoint = self.new_checkpoint(),
            self.try_term(),
            self.try_token(TokenKind::kTHEN),
            {
                let (_term, then_t) = all_of!(
                    "term then",
                    self.try_term(),
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
            self.try_back_ref(),
            {
                let (colon2_t, name_t) = self.parse_colon2_const()?;
                Ok(Builder::const_global(colon2_t, name_t, self.buffer()))
            },
            {
                let (primary_value, op_t, id_t) = all_of!(
                    "primary call_op [const/tIDENT]",
                    self.parse_primary_value(),
                    self.parse_call_op2(),
                    self.try_const_or_identifier(),
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
        parser.parse_arg_value().map(|arg_value| vec![*arg_value]),
        parser.parse_mrhs(),
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
    use crate::parser::{ParseError, Parser};

    #[test]
    fn test_opt_rescue1() {
        let mut parser = Parser::new(b"rescue");
        assert_eq!(parse_opt_rescue1(&mut parser), Err(ParseError::empty()));
    }
}
