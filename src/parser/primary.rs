use crate::{
    builder::Builder,
    parser::{
        macros::{all_of, maybe, one_of},
        ParseResult, Parser,
    },
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_primary(&mut self) -> ParseResult<Box<Node>> {
        let mut node = one_of!(
            "primary value",
            checkpoint = self.new_checkpoint(),
            self.parse_literal(),
            self.parse_strings(),
            self.parse_xstring(),
            self.parse_regexp(),
            self.parse_words(),
            self.parse_qwords(),
            self.parse_symbols(),
            self.parse_qsymbols(),
            self.parse_var_ref(),
            self.parse_back_ref(),
            {
                let id_t = self.try_token(TokenKind::tFID)?;

                todo!("call_method {:?}", id_t);
            },
            {
                let (begin_t, bodystmt, end_t) = all_of!(
                    "BEGIN { .. }",
                    self.try_token(TokenKind::kBEGIN),
                    self.try_bodystmt(),
                    self.expect_token(TokenKind::kEND),
                )?;

                todo!("begin {:?} {:?} {:?}", begin_t, bodystmt, end_t);
            },
            {
                let (lparen_t, stmt, rparen_t) = all_of!(
                    "( stmt )",
                    self.try_token(TokenKind::tLPAREN),
                    self.parse_stmt(),
                    self.parse_rparen(),
                )?;

                todo!("begin {:?} {:?} {:?}", lparen_t, stmt, rparen_t)
            },
            {
                let (colon2_t, name_t) = self.parse_colon2_const()?;
                Ok(Builder::const_global(colon2_t, name_t, self.buffer()))
            },
            self.parse_array(),
            self.parse_hash(),
            parse_keyword_cmd(self, TokenKind::kRETURN),
            self.parse_yield(),
            self.parse_defined(),
            parse_not_expr(self),
            {
                let (fcall, brace_block) = all_of!(
                    "fcall brace_block",
                    self.parse_fcall(),
                    self.parse_brace_block(),
                )?;

                todo!("fcall brace_block {:?} {:?}", fcall, brace_block)
            },
            // FIXME: this rule is left-recursive, this must be extracted to a post-rule
            // {
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
            // },
            self.parse_lambda(),
            self.parse_if_expr(),
            self.parse_unless_expr(),
            self.parse_while_expr(),
            self.parse_until_expr(),
            self.parse_case(),
            self.parse_for_loop(),
            self.parse_class(),
            self.parse_module(),
            self.parse_method(),
            parse_keyword_cmd(self, TokenKind::kBREAK),
            parse_keyword_cmd(self, TokenKind::kNEXT),
            parse_keyword_cmd(self, TokenKind::kREDO),
            parse_keyword_cmd(self, TokenKind::kRETRY),
        )?;

        loop {
            match maybe!(self.parse_colon2_const())? {
                Some((colon2_t, name_t)) => {
                    node = Builder::const_fetch(node, colon2_t, name_t, self.buffer());
                }
                None => {
                    // no match
                    break;
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
