use super::*;
use crate::builder::gvar;

impl<'a> Parser<'a> {
    pub(crate) fn parse_gvar(&mut self) -> Option<Box<Node<'a>>> {
        if self.current_token().value() != &TokenValue::tGVAR {
            return None;
        }

        let t_gvar = self.take_token();
        Some(gvar(t_gvar))
    }

    pub(crate) fn parse_back_ref(&mut self) -> Option<Box<Node<'a>>> {
        todo!()
    }

    pub(crate) fn parse_nth_ref(&mut self) -> Option<Box<Node<'a>>> {
        todo!()
    }
}
