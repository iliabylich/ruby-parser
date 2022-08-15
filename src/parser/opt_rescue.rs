use crate::{
    builder::Builder,
    parser::{macros::all_of, ParseError, ParseResult, Parser},
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
        self.one_of("then ...")
            .or_else(|| self.try_term())
            .or_else(|| self.try_token(TokenKind::kTHEN))
            .or_else(|| {
                let (_term, then_t) = all_of!(
                    "term then",
                    self.try_term(),
                    self.try_token(TokenKind::kTHEN),
                )?;
                Ok(then_t)
            })
            .stop()
    }

    pub(crate) fn parse_lhs(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("lhs")
            .or_else(|| self.parse_user_variable())
            .or_else(|| self.parse_keyword_variable())
            .or_else(|| self.try_back_ref())
            .or_else(|| {
                let (colon2_t, name_t) = self.parse_colon2_const()?;
                Ok(Builder::const_global(colon2_t, name_t, self.buffer()))
            })
            .or_else(|| {
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
            })
            .stop()
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
            parser
                .one_of("optional then")
                .or_else(|| parser.parse_then().map(|tok| Some(tok)))
                .or_else(|| Ok(None))
                .stop()
        },
        parser.try_compstmt(),
    )?;

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
            let (assoc_t, lhs) = all_of!(
                "exc var",
                parser.try_token(TokenKind::tASSOC),
                parser.parse_lhs(),
            )?;
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
