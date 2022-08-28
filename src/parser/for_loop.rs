use crate::{
    builder::Builder,
    parser::{
        macros::{all_of, one_of},
        ParseResult, Parser,
    },
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_for_loop(&mut self) -> ParseResult<Box<Node>> {
        let (for_t, iterator, in_t, (iteratee, do_t), body, end_t) = all_of!(
            "for loop",
            parse_k_for(self),
            parse_for_var(self),
            self.try_token(TokenKind::kIN),
            self.parse_expr_value_do(),
            self.try_compstmt(),
            self.parse_k_end(),
        )?;

        Ok(Builder::for_(
            for_t, iterator, in_t, iteratee, do_t, body, end_t,
        ))
    }
}

fn parse_for_var(parser: &mut Parser) -> ParseResult<Box<Node>> {
    one_of!(
        "for var",
        checkpoint = parser.new_checkpoint(),
        parser.parse_mlhs(),
        parser.parse_lhs(),
    )
}

fn parse_k_for(parser: &mut Parser) -> ParseResult<Token> {
    parser.try_token(TokenKind::kFOR)
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_for_lhs() {
        assert_parses!(
            Parser::parse_for_loop,
            b"for a in 1 do; 2; end",
            r#"
s(:for,
  s(:lvar, "a"),
  s(:int, "1"),
  s(:int, "2"))
            "#
        )
    }

    #[test]
    fn test_for_mlhs() {
        assert_parses!(
            Parser::parse_for_loop,
            b"for (a, b) in 1 do; 2; end",
            r#"
s(:for,
  s(:begin,
    s(:lvar, "a"),
    s(:lvar, "b")),
  s(:int, "1"),
  s(:int, "2"))
            "#
        )
    }
}
