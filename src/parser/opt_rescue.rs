use crate::{
    builder::{Builder, Constructor},
    parser::{ParseResult, ParseResultApi, Parser},
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
            match try_opt_rescue1(self).ignore_lookaheads()? {
                Some(node) => nodes.push(*node),
                None => {
                    // no match
                    break;
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
                let _term = self.try_term();
                let k_then = self.try_token(TokenKind::kTHEN)?;
                Ok(k_then)
            })
            .unwrap()
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
                let primary_value = self.try_primary_value()?;
                let op_t = self.try_call_op2()?;
                let id_t = self.try_const_or_identifier()?;
                panic!(
                    "primary_value call_op tIDENT {:?} {:?} {:?}",
                    primary_value, op_t, id_t
                )
            })
            .unwrap()
    }

    fn try_arg_value(&mut self) -> ParseResult<Box<Node>> {
        self.try_arg()
    }
}

fn try_opt_rescue1<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Box<Node>> {
    let rescue_t = parser.try_token(TokenKind::kRESCUE)?;
    let exc_list = try_exc_list(parser)?;
    let exc_var = try_exc_var(parser)?;
    let then = parser.try_then().ok();
    let compstmt = parser.try_compstmt()?;
    Ok(Builder::<C>::rescue_body(
        rescue_t,
        Some(exc_list),
        Some(exc_var),
        then,
        compstmt,
    ))
}

fn try_exc_list<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Vec<Node>> {
    parser
        .one_of("exceptions list")
        .or_else(|| parser.try_arg_value().map(|arg_value| vec![*arg_value]))
        .or_else(|| parser.try_mrhs())
        .unwrap()
}
fn try_exc_var<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<(Token, Box<Node>)> {
    let assoc_t = parser.try_token(TokenKind::tASSOC)?;
    let lhs = parser.try_lhs()?;
    Ok((assoc_t, lhs))
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
