use crate::{
    parser::{macros::all_of, ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_while_expr(&mut self) -> ParseResult<Box<Node>> {
        let (while_t, expr_value_do, compstmt, end_t) = all_of!(
            "while expr",
            self.parse_k_while(),
            self.parse_expr_value_do(),
            self.try_compstmt(),
            self.parse_k_end(),
        )?;

        todo!(
            "{:?} {:?} {:?} {:?}",
            while_t,
            expr_value_do,
            compstmt,
            end_t
        )
    }

    pub(crate) fn parse_until_expr(&mut self) -> ParseResult<Box<Node>> {
        let (until_t, expr_value_do, compstmt, end_t) = all_of!(
            "until expr",
            self.parse_k_until(),
            self.parse_expr_value_do(),
            self.try_compstmt(),
            self.parse_k_end(),
        )?;

        todo!(
            "{:?} {:?} {:?} {:?}",
            until_t,
            expr_value_do,
            compstmt,
            end_t
        )
    }

    fn parse_k_while(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kWHILE)
    }
    fn parse_k_until(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kUNTIL)
    }
}
