use crate::{
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_if_expr(&mut self) -> ParseResult<Box<Node>> {
        let (if_t, expr, then_t, compstmt, if_tail, end_t) = self
            .all_of("if expr")
            .and(|| self.parse_k_if())
            .and(|| self.parse_expr_value())
            .and(|| self.parse_then())
            .and(|| self.try_compstmt())
            .and(|| self.parse_if_tail())
            .and(|| self.parse_k_end())
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

    pub(crate) fn parse_unless_expr(&mut self) -> ParseResult<Box<Node>> {
        let (unless_t, expr, then_t, compstmt, opt_else, end_t) = self
            .all_of("if expr")
            .and(|| self.parse_k_unless())
            .and(|| self.parse_expr_value())
            .and(|| self.parse_then())
            .and(|| self.try_compstmt())
            .and(|| self.try_opt_else())
            .and(|| self.parse_k_end())
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

    fn parse_if_tail(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.parse_if_tail")
    }

    fn parse_k_if(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kIF)
    }

    fn parse_k_unless(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kUNLESS)
    }
}
