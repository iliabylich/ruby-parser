use crate::{
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn try_while_expr(&mut self) -> ParseResult<Box<Node>> {
        let (while_t, expr_value_do, compstmt, end_t) = self
            .all_of("while expr")
            .and(|| self.try_k_while())
            .and(|| self.try_expr_value_do())
            .and(|| self.try_compstmt())
            .and(|| self.try_k_end())
            .stop()?;

        todo!(
            "{:?} {:?} {:?} {:?}",
            while_t,
            expr_value_do,
            compstmt,
            end_t
        )
    }

    pub(crate) fn try_until_expr(&mut self) -> ParseResult<Box<Node>> {
        let (until_t, expr_value_do, compstmt, end_t) = self
            .all_of("until expr")
            .and(|| self.try_k_until())
            .and(|| self.try_expr_value_do())
            .and(|| self.try_compstmt())
            .and(|| self.try_k_end())
            .stop()?;

        todo!(
            "{:?} {:?} {:?} {:?}",
            until_t,
            expr_value_do,
            compstmt,
            end_t
        )
    }

    fn try_k_while(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kWHILE)
    }
    fn try_k_until(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kUNTIL)
    }
}
