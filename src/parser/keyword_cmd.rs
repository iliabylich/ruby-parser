use crate::{
    builder::Builder,
    parser::{base::Rule, Args, Value},
    Node, Parser, TokenKind,
};

pub(crate) struct KeywordCmd;
impl Rule for KeywordCmd {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is_one_of([
            TokenKind::kBREAK,
            TokenKind::kNEXT,
            TokenKind::kREDO,
            TokenKind::kRETRY,
            TokenKind::kRETURN,
            TokenKind::kYIELD,
            TokenKind::kDEFINED,
        ])
    }

    // TODO: double-check it after merging primary/expr/stmt
    fn parse(parser: &mut Parser) -> Self::Output {
        let keyword_t = parser.take_token();

        let node = match keyword_t.kind {
            TokenKind::kBREAK => Builder::break_(keyword_t, vec![]),
            TokenKind::kNEXT => Builder::next(keyword_t, vec![]),
            TokenKind::kREDO => Builder::redo(keyword_t),
            TokenKind::kRETRY => Builder::retry(keyword_t),
            TokenKind::kRETURN => Builder::return_(keyword_t, vec![]),
            TokenKind::kYIELD => {
                let lparen_t = if parser.current_token().is(TokenKind::tLPAREN) {
                    Some(parser.take_token())
                } else {
                    None
                };

                let args = if Args::starts_now(parser) {
                    Args::parse(parser)
                } else {
                    vec![]
                };

                let rparen_t = if lparen_t.is_some() {
                    Some(parser.expect_token(TokenKind::tRPAREN))
                } else {
                    None
                };

                Builder::yield_(keyword_t, lparen_t, args, rparen_t)
            }
            TokenKind::kDEFINED => {
                dbg!(parser.current_token());
                let lparen_t = parser.expect_token(TokenKind::tLPAREN);
                let value = Value::parse(parser);
                let rparen_t = parser.expect_token(TokenKind::tRPAREN);

                Builder::defined(keyword_t, Some(lparen_t), value, Some(rparen_t))
            }
            _ => unreachable!(),
        };

        node
    }
}
#[test]
fn test_keyword_cmd() {
    use crate::testing::assert_parses_rule;

    assert_parses_rule!(KeywordCmd, b"break", "s(:break)");
    assert_parses_rule!(KeywordCmd, b"next", "s(:next)");
    assert_parses_rule!(KeywordCmd, b"redo", "s(:redo)");
    assert_parses_rule!(KeywordCmd, b"retry", "s(:retry)");
    assert_parses_rule!(KeywordCmd, b"return", "s(:return)");
    // assert_parses_rule!(KeywordCmd, b"yield", "s(:yield)");
    // assert_parses_rule!(KeywordCmd, b"defined?(42)", "s(:defined, s(:int, \"42\""))");
}
