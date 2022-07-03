use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::TokenValue,
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_primary(&mut self) -> Option<Box<Node<'a>>> {
        let node = None
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
                let id_t = self.try_token(TokenValue::tFID)?;
                todo!("call_method {:?}", id_t);
            })
            .or_else(|| {
                let begin_t = self.try_token(TokenValue::kBEGIN)?;
                let bodystmt = self.try_bodystmt();
                let end_t = self.expect_token(TokenValue::kEND);
                todo!("begin {:?} {:?} {:?}", begin_t, bodystmt, end_t);
            })
            .or_else(|| {
                let lparen_t = self.try_token(TokenValue::tLPAREN)?;
                let stmt = self.try_stmt();
                if let Some(rparen_t) = self.try_rparen() {
                    todo!("begin {:?} {:?} {:?}", lparen_t, stmt, rparen_t)
                } else {
                    panic!("expected tRPAREN, got {:?}", self.current_token())
                }
            })
            .or_else(|| {
                let (colon2_t, const_t) = self.try_colon2_const()?;
                todo!("tCOLON2 tCONSTANT {:?} {:?}", colon2_t, const_t);
            })
            .or_else(|| self.try_array())
            .or_else(|| self.try_hash())
            .or_else(|| self.try_keyword_cmd(TokenValue::kRETURN))
            .or_else(|| self.try_yield())
            .or_else(|| self.try_defined())
            .or_else(|| self.try_not_expr())
            .or_else(|| {
                let fcall = self.try_fcall()?;
                if let Some(brace_block) = self.try_brace_block() {
                    todo!("fcall brace_block {:?} {:?}", fcall, brace_block)
                } else {
                    None
                }
            })
            .or_else(|| {
                let method_call = self.try_method_call()?;
                if let Some(brace_block) = self.try_brace_block() {
                    todo!(
                        "method_call brace_block {:?} {:?}",
                        method_call,
                        brace_block
                    )
                } else {
                    todo!("method_call {:?}", method_call)
                }
            })
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
            .or_else(|| self.try_keyword_cmd(TokenValue::kBREAK))
            .or_else(|| self.try_keyword_cmd(TokenValue::kNEXT))
            .or_else(|| self.try_keyword_cmd(TokenValue::kREDO))
            .or_else(|| self.try_keyword_cmd(TokenValue::kRETRY));

        let node = node?;

        while let Some((colon2_t, const_t)) = self.try_colon2_const() {
            todo!("append tCOLON2 tCONSTANT {:?} {:?}", colon2_t, const_t)
        }

        Some(node)
    }

    pub(crate) fn try_primary_value(&mut self) -> Option<Box<Node<'a>>> {
        self.try_primary()
    }

    // Helpers

    fn try_keyword_cmd(&mut self, expected: TokenValue<'a>) -> Option<Box<Node<'a>>> {
        let token = self.try_token(expected)?;
        todo!("keyword.cmd {:?}", token)
    }

    // kNOT tLPAREN2 expr rparen
    // kNOT tLPAREN2 rparen
    fn try_not_expr(&mut self) -> Option<Box<Node<'a>>> {
        let not_t = self.try_token(TokenValue::kNOT)?;
        let checkpoint = self.new_checkpoint();
        if let Some(lparen_t) = self.try_token(TokenValue::tLPAREN) {
            let expr = self.try_expr();
            if let Some(rparen_t) = self.try_rparen() {
                todo!(
                    "not_op {:?} {:?} {:?} {:?}",
                    not_t,
                    lparen_t,
                    expr,
                    rparen_t
                )
            } else {
                panic!("expected tRPAREN, got {:?}", self.current_token());
            }
        } else {
            self.restore_checkpoint(checkpoint);
            None
        }
    }
}
