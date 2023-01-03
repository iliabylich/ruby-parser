use crate::{
    parser::{
        base::{at_most_one_is_true, Maybe1, Rule, Unbox},
        Args, CallArgs, DotOrColon2T, DotT, MaybeBlock, OpT, ParenArgs,
    },
    Node, Parser, Token, TokenKind,
};

#[derive(Debug)]
pub(crate) enum CallTail {
    ConstAccess {
        colon2_t: Token,
        name_t: Token,
    },

    MethodCall {
        dot_t: Token,
        name_t: Option<Token>,
        lparen_t: Option<Token>,
        args: Vec<Node>,
        rparen_t: Option<Token>,

        block: Option<(Token, Option<Box<Node>>, Box<Node>, Token)>,
    },

    ArefArgs {
        lbrack_t: Token,
        args: Vec<Node>,
        rbrack_t: Token,

        block: Option<(Token, Option<Box<Node>>, Box<Node>, Token)>,
    },
}

impl Unbox for CallTail {
    type Output = Self;

    fn unbox(self) -> Self::Output {
        self
    }
}

impl Rule for CallTail {
    type Output = Self;

    fn starts_now(parser: &mut Parser) -> bool {
        let seen_any_space = parser.lexer.seen_nl || parser.lexer.seen_whitespace;

        at_most_one_is_true([
            parser.current_token().is(TokenKind::tCOLON2) && !seen_any_space,
            DotT::starts_now(parser),
            ArefArgs::starts_now(parser) && !seen_any_space,
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if ArefArgs::starts_now(parser) {
            let (lbrack_t, args, rbrack_t) = ArefArgs::parse(parser);
            let block = MaybeBlock::parse(parser);
            Self::ArefArgs {
                lbrack_t,
                args,
                rbrack_t,
                block,
            }
        } else {
            let dot_or_colon2_t = DotOrColon2T::parse(parser);
            let method_name_t = Maybe1::<MethodNameT>::parse(parser);

            let (lparen_t, args, rparen_t) = if method_name_t.is_none() {
                // `foo.()` / `foo::()`, can take only parenthesized args
                let (lparen_t, args, rparen_t) = ParenArgs::parse(parser);
                (Some(lparen_t), args, Some(rparen_t))
            } else {
                // normal `foo.bar` call, takes any args
                CallArgs::parse(parser)
            };

            if dot_or_colon2_t.is(TokenKind::tCOLON2)
                && matches!(
                    method_name_t,
                    Some(Token {
                        kind: TokenKind::tCONSTANT,
                        ..
                    })
                )
                && args.is_empty()
                && lparen_t.is_none()
                && rparen_t.is_none()
            {
                // ::CONST, without args and parentheses, it can't take any blocks because it's a const access
                Self::ConstAccess {
                    colon2_t: dot_or_colon2_t,
                    name_t: method_name_t.unwrap(),
                }
            } else {
                // some method call, can take any block
                let block = MaybeBlock::parse(parser);
                Self::MethodCall {
                    dot_t: dot_or_colon2_t,
                    name_t: method_name_t,
                    lparen_t,
                    args,
                    rparen_t,
                    block,
                }
            }
        }
    }
}

struct ArefArgs;
impl Rule for ArefArgs {
    type Output = (Token, Vec<Node>, Token);

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tLBRACK)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let lbrack_t = parser.take_token();
        let args = Maybe1::<Args>::parse(parser).unwrap_or_default();
        if parser.current_token().is(TokenKind::tCOMMA) {
            parser.skip_token()
        }
        let rbrack_t = parser.expect_token(TokenKind::tRBRACK);
        (lbrack_t, args, rbrack_t)
    }
}

struct AssignmentT;
impl Rule for AssignmentT {
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        let token = parser.current_token();
        token.is(TokenKind::tEQL) || token.is(TokenKind::tOP_ASGN)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        parser.take_token()
    }
}

pub(crate) struct MethodNameT;
impl Rule for MethodNameT {
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        let token = parser.current_token();

        at_most_one_is_true([
            token.is(TokenKind::tFID),
            token.is(TokenKind::tIDENTIFIER),
            token.is(TokenKind::tCONSTANT),
            OpT::starts_now(parser),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if Self::starts_now(parser) {
            parser.take_token()
        } else {
            unreachable!()
        }
    }
}

#[test]
fn test_call_tail_method() {
    use crate::{parser::Value, testing::assert_parses_rule};
    assert_parses_rule!(
        Value,
        b"42::foo(bar)",
        r#"
s(:send,
  s(:int, "42"), "foo",
  s(:begin,
    s(:send, nil, "bar")))
        "#
    )
}

#[test]
fn test_call_tail_const() {
    use crate::{parser::Value, testing::assert_parses_rule};
    assert_parses_rule!(
        Value,
        b"42::Foo",
        r#"
s(:const,
  s(:int, "42"), "Foo")
        "#
    )
}

#[test]
fn test_call_tail_aref() {
    use crate::{parser::Value, testing::assert_parses_rule};
    assert_parses_rule!(
        Value,
        b"42[true]",
        r#"
s(:index,
  s(:int, "42"),
  s(:true))
        "#
    )
}
