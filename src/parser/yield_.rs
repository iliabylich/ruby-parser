use crate::{
    builder::{Builder, KeywordCmd},
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn try_yield(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("yield with opt args")
            .or_else(|| {
                let (yield_t, lparen_t, args, rparen_t) = self
                    .all_of("yield(args)")
                    .and(|| self.try_token(TokenKind::kYIELD))
                    .and(|| self.expect_token(TokenKind::tLPAREN))
                    .and(|| self.try_call_args())
                    .and(|| self.try_rparen())
                    .stop()?;

                Ok(Builder::keyword_cmd(
                    KeywordCmd::Yield,
                    yield_t,
                    Some(lparen_t),
                    args,
                    Some(rparen_t),
                ))
            })
            .or_else(|| {
                let yield_t = self.try_token(TokenKind::kYIELD)?;
                Ok(Builder::keyword_cmd(
                    KeywordCmd::Yield,
                    yield_t,
                    None,
                    vec![],
                    None,
                ))
            })
            .stop()
    }
}
