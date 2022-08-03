use crate::{
    builder::Builder,
    parser::{ParseError, ParseResult, Parser},
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
                            return Err(ParseError::seq_error::<Vec<Node>, _>(
                                "opt rescue",
                                nodes,
                                error,
                            ));
                        }
                    }
                }
            }
        }
        Ok(nodes)
    }

    pub(crate) fn parse_then(&mut self) -> ParseResult<Token> {
        self.one_of("then ...")
            .or_else(|| self.parse_term())
            .or_else(|| self.try_token(TokenKind::kTHEN))
            .or_else(|| {
                let (_term, then_t) = self
                    .all_of("term then")
                    .and(|| self.parse_term())
                    .and(|| self.try_token(TokenKind::kTHEN))
                    .stop()?;
                Ok(then_t)
            })
            .stop()
    }

    pub(crate) fn parse_lhs(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("lhs")
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
                    .and(|| self.parse_call_op2())
                    .and(|| self.parse_const_or_identifier())
                    .stop()?;

                panic!(
                    "primary_value call_op tIDENT {:?} {:?} {:?}",
                    primary_value, op_t, id_t
                )
            })
            .stop()
    }

    pub(crate) fn parse_arg_value(&mut self) -> ParseResult<Box<Node>> {
        self.parse_arg()
    }
}

fn parse_opt_rescue1(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (rescue_t, exc_list, exc_var, then, compstmt) = parser
        .all_of("rescue1")
        .and(|| parser.try_token(TokenKind::kRESCUE))
        .and(|| parse_exc_list(parser))
        .and(|| try_exc_var(parser))
        .and(|| {
            parser
                .one_of("optional then")
                .or_else(|| parser.parse_then().map(|tok| Some(tok)))
                .or_else(|| Ok(None))
                .stop()
        })
        .and(|| parser.try_compstmt())
        .stop()?;

    Ok(Builder::rescue_body(
        rescue_t, exc_list, exc_var, then, compstmt,
    ))
}

fn parse_exc_list(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    parser
        .one_of("exceptions list")
        .or_else(|| parser.parse_arg_value().map(|arg_value| vec![*arg_value]))
        .or_else(|| parser.parse_mrhs())
        .stop()
}
fn try_exc_var(parser: &mut Parser) -> ParseResult<Option<(Token, Box<Node>)>> {
    parser
        .one_of("[exc var]")
        .or_else(|| {
            let (assoc_t, lhs) = parser
                .all_of("exc var")
                .and(|| parser.try_token(TokenKind::tASSOC))
                .and(|| parser.parse_lhs())
                .stop()?;
            Ok(Some((assoc_t, lhs)))
        })
        .or_else(|| Ok(None))
        .stop()
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
