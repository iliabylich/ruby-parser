use super::*;

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn parse_undef(&mut self) -> Option<Box<Node<'a>>> {
        let undef_t = self.try_token(TokenValue::kUNDEF)?;
        let undef_list = self.parse_undef_list();
        panic!("undef({:?}, {:?})", undef_t, undef_list)
    }

    fn parse_undef_list(&mut self) -> Vec<Node<'a>> {
        let mut undef_list = vec![];
        if let Some(fitem) = self.parse_fitem() {
            undef_list.push(*fitem);
        }
        loop {
            if self.current_token().value() != &TokenValue::tCOMMA {
                break;
            }
            match self.parse_fitem() {
                Some(fitem) => undef_list.push(*fitem),
                None => panic!("expected fitem, got {:?}", self.current_token()),
            }
        }
        undef_list
    }
}