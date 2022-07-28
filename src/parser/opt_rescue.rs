use crate::{
    builder::{Builder, Constructor},
    parser::{ParseError, ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    // This rule can be `none`
    pub(crate) fn try_opt_rescue(&mut self) -> ParseResult<Vec<Node>> {
        let mut nodes = vec![];
        loop {
            match try_opt_rescue1(self) {
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

    pub(crate) fn try_then(&mut self) -> ParseResult<Token> {
        self.one_of("then ...")
            .or_else(|| self.try_term())
            .or_else(|| self.try_token(TokenKind::kTHEN))
            .or_else(|| {
                let (_term, then_t) = self
                    .all_of("term then")
                    .and(|| self.try_term())
                    .and(|| self.try_token(TokenKind::kTHEN))
                    .stop()?;
                Ok(then_t)
            })
            .stop()
    }

    pub(crate) fn try_lhs(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("lhs")
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
                    .and(|| self.try_call_op2())
                    .and(|| self.try_const_or_identifier())
                    .stop()?;

                panic!(
                    "primary_value call_op tIDENT {:?} {:?} {:?}",
                    primary_value, op_t, id_t
                )
            })
            .stop()
    }

    pub(crate) fn try_arg_value(&mut self) -> ParseResult<Box<Node>> {
        self.try_arg()
    }
}

fn try_opt_rescue1<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Box<Node>> {
    let (rescue_t, exc_list, exc_var, then, compstmt) = parser
        .all_of("rescue1")
        .and(|| parser.try_token(TokenKind::kRESCUE))
        .and(|| try_exc_list(parser))
        .and(|| try_exc_var(parser))
        .and(|| {
            parser
                .one_of("optional then")
                .or_else(|| parser.try_then().map(|tok| Some(tok)))
                .or_else(|| Ok(None))
                .stop()
        })
        .and(|| parser.try_compstmt())
        .stop()?;

    Ok(Builder::<C>::rescue_body(
        rescue_t, exc_list, exc_var, then, compstmt,
    ))
}

fn try_exc_list<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Vec<Node>> {
    parser
        .one_of("exceptions list")
        .or_else(|| parser.try_arg_value().map(|arg_value| vec![*arg_value]))
        .or_else(|| parser.try_mrhs())
        .stop()
}
fn try_exc_var<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Option<(Token, Box<Node>)>> {
    parser
        .one_of("[exc var]")
        .or_else(|| {
            let (assoc_t, lhs) = parser
                .all_of("exc var")
                .and(|| parser.try_token(TokenKind::tASSOC))
                .and(|| parser.try_lhs())
                .stop()?;
            Ok(Some((assoc_t, lhs)))
        })
        .or_else(|| Ok(None))
        .stop()
}

#[cfg(test)]
mod tests {
    use super::try_opt_rescue1;
    use crate::parser::{ParseError, RustParser};

    #[test]
    fn test_opt_rescue1() {
        let mut parser = RustParser::new(b"rescue");
        assert_eq!(try_opt_rescue1(&mut parser), Err(ParseError::empty()));
    }
}
