use crate::{
    builder::Builder,
    parser::{macros::all_of, ParseError, ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_primary(&mut self) -> ParseResult<Box<Node>> {
        let mut node = self
            .one_of("primary value")
            .or_else(|| self.parse_literal())
            .or_else(|| self.parse_strings())
            .or_else(|| self.parse_xstring())
            .or_else(|| self.parse_regexp())
            .or_else(|| self.parse_words())
            .or_else(|| self.parse_qwords())
            .or_else(|| self.parse_symbols())
            .or_else(|| self.parse_qsymbols())
            .or_else(|| self.parse_var_ref())
            .or_else(|| self.try_back_ref())
            .or_else(|| {
                let id_t = self.try_token(TokenKind::tFID)?;

                todo!("call_method {:?}", id_t);
            })
            .or_else(|| {
                let (begin_t, bodystmt, end_t) = all_of!(
                    "BEGIN { .. }",
                    self.try_token(TokenKind::kBEGIN),
                    self.try_bodystmt(),
                    self.expect_token(TokenKind::kEND),
                )?;

                todo!("begin {:?} {:?} {:?}", begin_t, bodystmt, end_t);
            })
            .or_else(|| {
                let (lparen_t, stmt, rparen_t) = all_of!(
                    "( stmt )",
                    self.try_token(TokenKind::tLPAREN),
                    self.parse_stmt(),
                    self.parse_rparen(),
                )?;

                todo!("begin {:?} {:?} {:?}", lparen_t, stmt, rparen_t)
            })
            .or_else(|| {
                let (colon2_t, name_t) = self.parse_colon2_const()?;
                Ok(Builder::const_global(colon2_t, name_t, self.buffer()))
            })
            .or_else(|| self.parse_array())
            .or_else(|| self.parse_hash())
            .or_else(|| parse_keyword_cmd(self, TokenKind::kRETURN))
            .or_else(|| self.parse_yield())
            .or_else(|| self.parse_defined())
            .or_else(|| parse_not_expr(self))
            .or_else(|| {
                let (fcall, brace_block) = all_of!(
                    "fcall brace_block",
                    self.parse_fcall(),
                    self.parse_brace_block(),
                )?;

                todo!("fcall brace_block {:?} {:?}", fcall, brace_block)
            })
            // FIXME: this rule is left-recursive, this must be extracted to a post-rule
            // .or_else(|| {
            //     let method_call = self.parse_method_call()?;
            //     if let Ok(brace_block) = self.parse_brace_block() {
            //         todo!(
            //             "method_call brace_block {:?} {:?}",
            //             method_call,
            //             brace_block
            //         )
            //     } else {
            //         todo!("method_call {:?}", method_call)
            //     }
            // })
            .or_else(|| self.parse_lambda())
            .or_else(|| self.parse_if_expr())
            .or_else(|| self.parse_unless_expr())
            .or_else(|| self.parse_while_expr())
            .or_else(|| self.parse_until_expr())
            .or_else(|| self.parse_case())
            .or_else(|| self.parse_for_loop())
            .or_else(|| self.parse_class())
            .or_else(|| self.parse_module())
            .or_else(|| self.parse_method())
            .or_else(|| parse_keyword_cmd(self, TokenKind::kBREAK))
            .or_else(|| parse_keyword_cmd(self, TokenKind::kNEXT))
            .or_else(|| parse_keyword_cmd(self, TokenKind::kREDO))
            .or_else(|| parse_keyword_cmd(self, TokenKind::kRETRY))
            .compact()
            .stop()?;

        loop {
            match self.parse_colon2_const() {
                Ok((colon2_t, name_t)) => {
                    node = Builder::const_fetch(node, colon2_t, name_t, self.buffer());
                }
                Err(error) => {
                    match error.strip_lookaheads() {
                        None => {
                            // no match
                            break;
                        }
                        Some(error) => {
                            return Err(ParseError::seq_error("primary -> ::CONST", node, error));
                        }
                    }
                }
            }
        }

        Ok(node)
    }

    pub(crate) fn parse_primary_value(&mut self) -> ParseResult<Box<Node>> {
        self.parse_primary()
    }
}

fn parse_keyword_cmd(parser: &mut Parser, expected: TokenKind) -> ParseResult<Box<Node>> {
    let token = parser.try_token(expected)?;
    todo!("keyword.cmd {:?}", token)
}

// kNOT tLPAREN2 expr rparen
// kNOT tLPAREN2 rparen
fn parse_not_expr(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (not_t, lparen_t, expr, rparen_t) = all_of!(
        "not ( [expr] )",
        parser.try_token(TokenKind::kNOT),
        parser.try_token(TokenKind::tLPAREN),
        parser.parse_expr(),
        parser.parse_rparen(),
    )?;

    todo!(
        "not_op {:?} {:?} {:?} {:?}",
        not_t,
        lparen_t,
        expr,
        rparen_t
    )
}
