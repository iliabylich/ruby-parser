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
    pub(crate) fn try_yield(&mut self) -> Option<Box<Node<'a>>> {
        let yield_t = self.try_token(TokenValue::kYIELD)?;
        if let Some(lparen_t) = self.try_token(TokenValue::tLPAREN) {
            let call_args = self.parse_call_args();
            if let Some(rparen_t) = self.try_rparen() {
                todo!(
                    "yield {:?} {:?} {:?} {:?}",
                    yield_t,
                    lparen_t,
                    call_args,
                    rparen_t
                );
            } else {
                panic!("expected tRPAREN, got {:?}", self.current_token())
            }
        } else {
            // just `yield`
            todo!("yield {:?}", yield_t)
        }
    }
}
