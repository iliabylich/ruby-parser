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
    pub(crate) fn try_if_expr(&mut self) -> ParseResult<Box<Node>> {
        let (if_t, expr, then_t, compstmt, if_tail, end_t) = self
            .all_of("if expr")
            .and(|| self.try_k_if())
            .and(|| self.try_expr_value())
            .and(|| self.try_then())
            .and(|| self.try_compstmt())
            .and(|| self.try_if_tail())
            .and(|| self.try_k_end())
            .stop()?;

        todo!(
            "{:?} {:?} {:?} {:?} {:?} {:?}",
            if_t,
            expr,
            then_t,
            compstmt,
            if_tail,
            end_t
        )
    }

    pub(crate) fn try_unless_expr(&mut self) -> ParseResult<Box<Node>> {
        let (unless_t, expr, then_t, compstmt, opt_else, end_t) = self
            .all_of("if expr")
            .and(|| self.try_k_unless())
            .and(|| self.try_expr_value())
            .and(|| self.try_then())
            .and(|| self.try_compstmt())
            .and(|| self.try_opt_else())
            .and(|| self.try_k_end())
            .stop()?;

        todo!(
            "{:?} {:?} {:?} {:?} {:?} {:?}",
            unless_t,
            expr,
            then_t,
            compstmt,
            opt_else,
            end_t
        )
    }

    fn try_if_tail(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.try_if_tail")
    }

    fn try_k_if(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kIF)
    }

    fn try_k_unless(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kUNLESS)
    }
}
