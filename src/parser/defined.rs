use crate::{
    parser::{macros::all_of, ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_defined(&mut self) -> ParseResult<Box<Node>> {
        let (defined_t, _nl, lparen_t, expr, rparen_t) = all_of!(
            "defined? value",
            self.try_token(TokenKind::kDEFINED),
            self.try_opt_nl(),
            self.expect_token(TokenKind::tLPAREN),
            self.parse_expr(),
            self.expect_token(TokenKind::tRPAREN),
        )?;

        todo!(
            "defined {:?} {:?} {:?} {:?}",
            defined_t,
            lparen_t,
            expr,
            rparen_t
        )
    }
}
