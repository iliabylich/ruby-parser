use crate::{
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_for_loop(&mut self) -> ParseResult<Box<Node>> {
        let (for_t, var, in_t, (value, do_t), body, end_t) = self
            .all_of("for loop")
            .and(|| self.parse_k_for())
            .and(|| self.parse_for_var())
            .and(|| self.try_token(TokenKind::kIN))
            .and(|| self.parse_expr_value_do())
            .and(|| self.try_compstmt())
            .and(|| self.parse_k_end())
            .stop()?;

        panic!(
            "{:?} {:?} {:?} {:?} {:?} {:?} {:?}",
            for_t, var, in_t, value, do_t, body, end_t
        );
    }

    fn parse_for_var(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.parse_for_var")
    }

    fn parse_k_for(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kFOR)
    }
}
