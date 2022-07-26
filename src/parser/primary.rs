use crate::{
    builder::Constructor,
    parser::{ParseResult, ParseResultApi, Parser},
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_primary(&mut self) -> ParseResult<Box<Node>> {
        let node = self
            .one_of("primary value")
            .or_else(|| self.try_literal())
            .or_else(|| self.try_strings())
            .or_else(|| self.try_xstring())
            .or_else(|| self.try_regexp())
            .or_else(|| self.try_words())
            .or_else(|| self.try_qwords())
            .or_else(|| self.try_symbols())
            .or_else(|| self.try_qsymbols())
            .or_else(|| self.try_var_ref())
            .or_else(|| self.try_back_ref())
            .or_else(|| {
                let id_t = self.try_token(TokenKind::tFID)?;

                todo!("call_method {:?}", id_t);
            })
            .or_else(|| {
                let (begin_t, bodystmt, end_t) = self
                    .all_of("BEGIN { .. }")
                    .and(|| self.try_token(TokenKind::kBEGIN))
                    .and(|| self.try_bodystmt())
                    .and(|| self.expect_token(TokenKind::kEND))
                    .unwrap()?;

                todo!("begin {:?} {:?} {:?}", begin_t, bodystmt, end_t);
            })
            .or_else(|| {
                let (lparen_t, stmt, rparen_t) = self
                    .all_of("( stmt )")
                    .and(|| self.try_token(TokenKind::tLPAREN))
                    .and(|| self.try_stmt())
                    .and(|| self.try_rparen())
                    .unwrap()?;

                todo!("begin {:?} {:?} {:?}", lparen_t, stmt, rparen_t)
            })
            .or_else(|| {
                let (colon2_t, const_t) = self.try_colon2_const()?;
                todo!("tCOLON2 tCONSTANT {:?} {:?}", colon2_t, const_t);
            })
            .or_else(|| self.try_array())
            .or_else(|| self.try_hash())
            .or_else(|| try_keyword_cmd(self, TokenKind::kRETURN))
            .or_else(|| self.try_yield())
            .or_else(|| self.try_defined())
            .or_else(|| try_not_expr(self))
            .or_else(|| {
                let (fcall, brace_block) = self
                    .all_of("fcall brace_block")
                    .and(|| self.try_fcall())
                    .and(|| self.try_brace_block())
                    .unwrap()?;

                todo!("fcall brace_block {:?} {:?}", fcall, brace_block)
            })
            // FIXME: this rule is left-recursive, this must be extracted to a post-rule
            // .or_else(|| {
            //     let method_call = self.try_method_call()?;
            //     if let Ok(brace_block) = self.try_brace_block() {
            //         todo!(
            //             "method_call brace_block {:?} {:?}",
            //             method_call,
            //             brace_block
            //         )
            //     } else {
            //         todo!("method_call {:?}", method_call)
            //     }
            // })
            .or_else(|| self.try_lambda())
            .or_else(|| self.try_if_expr())
            .or_else(|| self.try_unless_expr())
            .or_else(|| self.try_while_expr())
            .or_else(|| self.try_until_expr())
            .or_else(|| self.try_case())
            .or_else(|| self.try_for_loop())
            .or_else(|| self.try_class())
            .or_else(|| self.try_module())
            .or_else(|| self.try_method())
            .or_else(|| try_keyword_cmd(self, TokenKind::kBREAK))
            .or_else(|| try_keyword_cmd(self, TokenKind::kNEXT))
            .or_else(|| try_keyword_cmd(self, TokenKind::kREDO))
            .or_else(|| try_keyword_cmd(self, TokenKind::kRETRY))
            .unwrap()?;

        loop {
            match self.try_colon2_const().ignore_lookaheads()? {
                Some((colon2_t, const_t)) => {
                    todo!("append tCOLON2 tCONSTANT {:?} {:?}", colon2_t, const_t)
                }
                None => {
                    // no match
                    break;
                }
            }
        }

        Ok(node)
    }

    pub(crate) fn try_primary_value(&mut self) -> ParseResult<Box<Node>> {
        self.try_primary()
    }
}

fn try_keyword_cmd<C: Constructor>(
    parser: &mut Parser<C>,
    expected: TokenKind,
) -> ParseResult<Box<Node>> {
    let token = parser.try_token(expected)?;
    todo!("keyword.cmd {:?}", token)
}

// kNOT tLPAREN2 expr rparen
// kNOT tLPAREN2 rparen
fn try_not_expr<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Box<Node>> {
    let (not_t, lparen_t, expr, rparen_t) = parser
        .all_of("not ( [expr] )")
        .and(|| parser.try_token(TokenKind::kNOT))
        .and(|| parser.try_token(TokenKind::tLPAREN))
        .and(|| parser.try_expr())
        .and(|| parser.try_rparen())
        .unwrap()?;

    todo!(
        "not_op {:?} {:?} {:?} {:?}",
        not_t,
        lparen_t,
        expr,
        rparen_t
    )
}
