use crate::{
    builder::Constructor,
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_yield(&mut self) -> ParseResult<Box<Node>> {
        let yield_t = self.try_token(TokenKind::kYIELD)?;
        if let Ok(lparen_t) = self.try_token(TokenKind::tLPAREN) {
            let call_args = self.try_call_args()?;
            let rparen_t = self.try_rparen()?;
            todo!(
                "yield {:?} {:?} {:?} {:?}",
                yield_t,
                lparen_t,
                call_args,
                rparen_t
            );
        } else {
            // just `yield`
            todo!("yield {:?}", yield_t)
        }
    }
}
