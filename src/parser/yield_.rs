use crate::{
    builder::{Builder, KeywordCmd},
    parser::{
        macros::{all_of, one_of},
        ParseResult, Parser,
    },
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_yield(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "yield with opt args",
            checkpoint = self.new_checkpoint(),
            {
                let (yield_t, lparen_t, args, rparen_t) = all_of!(
                    "yield(args)",
                    self.try_token(TokenKind::kYIELD),
                    self.expect_token(TokenKind::tLPAREN),
                    self.parse_call_args(),
                    self.parse_rparen(),
                )?;

                Ok(Builder::keyword_cmd(
                    KeywordCmd::Yield,
                    yield_t,
                    Some(lparen_t),
                    args,
                    Some(rparen_t),
                ))
            },
            {
                let yield_t = self.try_token(TokenKind::kYIELD)?;
                Ok(Builder::keyword_cmd(
                    KeywordCmd::Yield,
                    yield_t,
                    None,
                    vec![],
                    None,
                ))
            },
        )
    }
}
