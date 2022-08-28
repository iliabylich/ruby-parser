use crate::{
    parser::{macros::all_of, ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_while_expr(&mut self) -> ParseResult<Box<Node>> {
        let (while_t, expr_value_do, compstmt, end_t) = all_of!(
            "while expr",
            parse_k_while(self),
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
            parse_k_until(self),
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
}

fn parse_k_while(parser: &mut Parser) -> ParseResult<Token> {
    parser.try_token(TokenKind::kWHILE)
}
fn parse_k_until(parser: &mut Parser) -> ParseResult<Token> {
    parser.try_token(TokenKind::kUNTIL)
}
