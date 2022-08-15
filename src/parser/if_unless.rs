use crate::{
    parser::{macros::all_of, ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_if_expr(&mut self) -> ParseResult<Box<Node>> {
        let (if_t, expr, then_t, compstmt, if_tail, end_t) = all_of!(
            "if expr",
            self.parse_k_if(),
            self.parse_expr_value(),
            self.parse_then(),
            self.try_compstmt(),
            self.parse_if_tail(),
            self.parse_k_end(),
        )?;

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
        let (unless_t, expr, then_t, compstmt, opt_else, end_t) = all_of!(
            "if expr",
            self.parse_k_unless(),
            self.parse_expr_value(),
            self.parse_then(),
            self.try_compstmt(),
            self.try_opt_else(),
            self.parse_k_end(),
        )?;

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
