use crate::{
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_defined(&mut self) -> ParseResult<Box<Node>> {
        let (defined_t, _nl, lparen_t, expr, rparen_t) = self
            .all_of("defined? value")
            .and(|| self.try_token(TokenKind::kDEFINED))
            .and(|| self.try_opt_nl())
            .and(|| self.expect_token(TokenKind::tLPAREN))
            .and(|| self.parse_expr())
            .and(|| self.expect_token(TokenKind::tRPAREN))
            .stop()?;

        todo!(
            "defined {:?} {:?} {:?} {:?}",
            defined_t,
            lparen_t,
            expr,
            rparen_t
        )
    }
}
