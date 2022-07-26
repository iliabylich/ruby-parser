use crate::{
    builder::Constructor,
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_for_loop(&mut self) -> ParseResult<Box<Node>> {
        let (for_t, var, in_t, (value, do_t), body, end_t) = self
            .all_of("for loop")
            .and(|| self.try_k_for())
            .and(|| self.try_for_var())
            .and(|| self.try_token(TokenKind::kIN))
            .and(|| self.try_expr_value_do())
            .and(|| self.try_compstmt())
            .and(|| self.try_k_end())
            .unwrap()?;

        panic!(
            "{:?} {:?} {:?} {:?} {:?} {:?} {:?}",
            for_t, var, in_t, value, do_t, body, end_t
        );
    }

    fn try_for_var(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.try_for_var")
    }

    fn try_k_for(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kFOR)
    }
}
