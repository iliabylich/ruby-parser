use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::{Token, TokenValue},
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn parse_opt_rescue(&mut self) -> Vec<Node<'a>> {
        let mut nodes = vec![];
        while let Some(node) = self.try_opt_rescue1() {
            nodes.push(*node)
        }
        nodes
    }

    fn try_opt_rescue1(&mut self) -> Option<Box<Node<'a>>> {
        let rescue_t = self.try_token(TokenValue::kRESCUE)?;
        let exc_list = self.try_exc_list();
        let exc_var = self.try_exc_var();
        let then = self.try_then();
        let compstmt = self.try_compstmt();
        Some(Builder::<C>::rescue_body(
            rescue_t, exc_list, exc_var, then, compstmt,
        ))
    }

    fn try_exc_list(&mut self) -> Option<Vec<Node<'a>>> {
        None.or_else(|| self.try_arg_value().map(|arg_value| vec![*arg_value]))
            .or_else(|| self.try_mrhs())
    }
    fn try_exc_var(&mut self) -> Option<(Token<'a>, Box<Node<'a>>)> {
        let assoc_t = self.try_token(TokenValue::tASSOC)?;
        if let Some(lhs) = self.try_lhs() {
            Some((assoc_t, lhs))
        } else {
            panic!("error: expected LHS after =>")
        }
    }

    fn try_then(&mut self) -> Option<Token<'a>> {
        None.or_else(|| self.try_term())
            .or_else(|| self.try_token(TokenValue::kTHEN))
            .or_else(|| {
                let checkpoint = self.new_checkpoint();
                if let Some(_term) = self.try_term() {
                    if let Some(k_then) = self.try_token(TokenValue::kTHEN) {
                        return Some(k_then);
                    }
                }
                self.restore_checkpoint(checkpoint);
                None
            })
    }

    pub(crate) fn try_lhs(&mut self) -> Option<Box<Node<'a>>> {
        None.or_else(|| self.try_user_variable())
            .or_else(|| self.try_keyword_variable())
            .or_else(|| self.try_back_ref())
            .or_else(|| {
                let colon2_t = self.try_token(TokenValue::tCOLON2)?;
                let const_t = self.expect_token(TokenValue::tCONSTANT);
                panic!("const {:?} {:?}", colon2_t, const_t)
            })
            .or_else(|| {
                let checkpoint = self.new_checkpoint();
                if let Some(primary_value) = self.try_primary_value() {
                    if let Some(op_t) = self.try_call_op() {
                        let id_t = self
                            .try_const_or_identifier()
                            .expect("expected tCONST or tIDDENT");
                        panic!(
                            "primary_value call_op tIDENT {:?} {:?} {:?}",
                            primary_value, op_t, id_t
                        )
                    } else if let Some(colon2_t) = self.try_token(TokenValue::tCOLON2) {
                        let const_t = self
                            .try_const_or_identifier()
                            .expect("expected tCONST or tIDDENT");

                        panic!(
                            "primary_value tCOLON2 tCONSTANT {:?} {:?} {:?}",
                            primary_value, colon2_t, const_t
                        )
                    }
                }

                self.restore_checkpoint(checkpoint);
                None
            })
    }

    fn try_arg_value(&mut self) -> Option<Box<Node<'a>>> {
        self.try_arg()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::RustParser;

    #[test]
    fn test_opt_rescue1() {
        let mut parser = RustParser::new(b"rescue");
        assert_eq!(parser.try_opt_rescue1(), None);
    }
}
