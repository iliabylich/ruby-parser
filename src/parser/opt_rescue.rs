use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::{Token, TokenKind},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    // This rule can be `none`
    pub(crate) fn try_opt_rescue(&mut self) -> Vec<Node> {
        let mut nodes = vec![];
        while let Some(node) = try_opt_rescue1(self) {
            nodes.push(*node)
        }
        nodes
    }

    pub(crate) fn try_then(&mut self) -> Option<Token> {
        None.or_else(|| self.try_term())
            .or_else(|| self.try_token(TokenKind::kTHEN))
            .or_else(|| {
                let checkpoint = self.new_checkpoint();
                if let Some(_term) = self.try_term() {
                    if let Some(k_then) = self.try_token(TokenKind::kTHEN) {
                        return Some(k_then);
                    }
                }
                checkpoint.restore();
                None
            })
    }

    pub(crate) fn try_lhs(&mut self) -> Option<Box<Node>> {
        None.or_else(|| self.try_user_variable())
            .or_else(|| self.try_keyword_variable())
            .or_else(|| self.try_back_ref())
            .or_else(|| {
                let (colon2_t, const_t) = self.try_colon2_const()?;
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
                    } else if let Some(colon2_t) = self.try_token(TokenKind::tCOLON2) {
                        let const_t = self
                            .try_const_or_identifier()
                            .expect("expected tCONST or tIDDENT");

                        panic!(
                            "primary_value tCOLON2 tCONSTANT {:?} {:?} {:?}",
                            primary_value, colon2_t, const_t
                        )
                    }
                }

                checkpoint.restore();
                None
            })
    }

    fn try_arg_value(&mut self) -> Option<Box<Node>> {
        self.try_arg()
    }
}

fn try_opt_rescue1<C: Constructor>(parser: &mut Parser<C>) -> Option<Box<Node>> {
    let rescue_t = parser.try_token(TokenKind::kRESCUE)?;
    let exc_list = try_exc_list(parser);
    let exc_var = try_exc_var(parser);
    let then = parser.try_then();
    let compstmt = parser.try_compstmt();
    Some(Builder::<C>::rescue_body(
        rescue_t, exc_list, exc_var, then, compstmt,
    ))
}

fn try_exc_list<C: Constructor>(parser: &mut Parser<C>) -> Option<Vec<Node>> {
    None.or_else(|| parser.try_arg_value().map(|arg_value| vec![*arg_value]))
        .or_else(|| parser.try_mrhs())
}
fn try_exc_var<C: Constructor>(parser: &mut Parser<C>) -> Option<(Token, Box<Node>)> {
    let assoc_t = parser.try_token(TokenKind::tASSOC)?;
    if let Some(lhs) = parser.try_lhs() {
        Some((assoc_t, lhs))
    } else {
        panic!("error: expected LHS after =>")
    }
}

#[cfg(test)]
mod tests {
    use super::try_opt_rescue1;
    use crate::parser::RustParser;

    #[test]
    fn test_opt_rescue1() {
        let mut parser = RustParser::new(b"rescue");
        assert_eq!(try_opt_rescue1(&mut parser), None);
    }
}
