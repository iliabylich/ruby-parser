use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_yield(&mut self) -> Option<Box<Node>> {
        let yield_t = self.try_token(TokenKind::kYIELD)?;
        if let Some(lparen_t) = self.try_token(TokenKind::tLPAREN) {
            let call_args = self.try_call_args().unwrap_or_else(|| vec![]);
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
