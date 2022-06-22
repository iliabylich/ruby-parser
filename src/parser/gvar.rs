use super::*;
use crate::builder::{Builder, Constructor};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn parse_gvar(&mut self) -> Option<Box<Node<'a>>> {
        if self.current_token().value() != &TokenValue::tGVAR {
            return None;
        }

        let t_gvar = self.take_token();
        Some(Builder::<C>::gvar(
            t_gvar,
            self.lexer.buffer.for_lookahead(),
        ))
    }

    pub(crate) fn parse_back_ref(&mut self) -> Option<Box<Node<'a>>> {
        if self.current_token().value() != &TokenValue::tBACK_REF {
            return None;
        }

        let t_back_ref = self.take_token();
        Some(Builder::<C>::back_ref(t_back_ref))
    }

    pub(crate) fn parse_nth_ref(&mut self) -> Option<Box<Node<'a>>> {
        todo!()
    }
}
