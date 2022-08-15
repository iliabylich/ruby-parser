use crate::{
    parser::{macros::all_of, ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_for_loop(&mut self) -> ParseResult<Box<Node>> {
        let (for_t, var, in_t, (value, do_t), body, end_t) = all_of!(
            "for loop",
            self.parse_k_for(),
            self.parse_for_var(),
            self.try_token(TokenKind::kIN),
            self.parse_expr_value_do(),
            self.try_compstmt(),
            self.parse_k_end(),
        )?;

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
