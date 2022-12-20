use crate::{
    builder::Builder,
    parser::{
        base::{ExactToken, Maybe1, Maybe2, Rule},
        Compstmt, TermT,
    },
    token::{Token, TokenKind},
    Node, Parser,
};

pub(crate) struct IfStmt;
impl Rule for IfStmt {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kIF)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        todo!()
    }
}

pub(crate) struct UnlessStmt;
impl Rule for UnlessStmt {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kUNLESS)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        todo!()
    }
}

type ElseT = ExactToken<{ TokenKind::kELSE as u8 }>;
pub(crate) type OptElse = Maybe2<ElseT, Compstmt>;

pub(crate) struct Then;
impl Rule for Then {
    type Output = Option<Token>;

    fn starts_now(_parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        type MaybeTermT = Maybe1<TermT>;
        type ThenT = ExactToken<{ TokenKind::kTHEN as u8 }>;
        type MaybeThenT = Maybe1<ThenT>;

        let _term = MaybeTermT::parse(parser);
        let then_t = MaybeThenT::parse(parser);
        then_t
    }
}

#[cfg(test)]
mod tests {
    use super::{IfStmt, UnlessStmt};
    use crate::testing::assert_parses_rule;

    #[test]
    fn test_if() {
        debug_assert!(false, "implement me");

        assert_parses_rule!(
            IfStmt,
            b"if 1; 2; else; 3; end",
            r#"
s(:if,
  s(:int, "1"),
  s(:int, "2"),
  s(:int, "3"))
            "#
        )
    }

    #[test]
    fn test_unless() {
        debug_assert!(false, "implement me");

        assert_parses_rule!(
            UnlessStmt,
            b"unless 1; 2; else; 3; end",
            r#"
s(:if,
  s(:int, "1"),
  s(:int, "3"),
  s(:int, "2"))
            "#
        )
    }
}
