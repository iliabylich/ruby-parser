use crate::{
    builder::{ArgsType, Builder},
    parser::{
        Args, Array, BackRef, Case, Class, Expr, ForLoop, Hash, IfStmt, Lambda, Literal, MethodDef,
        Module, OperationT, ParseResult, Rule, UnlessStmt, VarRef,
    },
    Node, Parser, Token, TokenKind,
};

pub(crate) struct Primary;
impl Rule for Primary {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        PrimaryHead::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let mut node = PrimaryHead::parse(parser).unwrap();

        while PrimaryTail::starts_now(parser) {
            let tail = PrimaryTail::parse(parser).unwrap();

            match tail {
                PrimaryTail::ConstAccess { colon2_t, name_t } => {
                    node = Builder::const_fetch(node, colon2_t, name_t, parser.buffer())
                }
                PrimaryTail::MethodCall {
                    dot_t,
                    selector_t,
                    lparen_t,
                    args,
                    rparen_t,
                    block_begin_t,
                    block_args,
                    block_body,
                    block_end_t,
                } => {
                    node = Builder::call_method(
                        Some(node),
                        dot_t,
                        selector_t,
                        lparen_t,
                        args,
                        rparen_t,
                        parser.buffer(),
                    );
                    node = Builder::block(node, block_begin_t, block_args, block_body, block_end_t)
                }
                PrimaryTail::IndexCall {
                    lbrack_t,
                    indexes,
                    rbrack_t,
                    block_begin_t,
                    block_args,
                    block_body,
                    block_end_t,
                } => {
                    node = Builder::index(node, lbrack_t, indexes, rbrack_t);
                    node = Builder::block(node, block_begin_t, block_args, block_body, block_end_t)
                }
            }
        }

        Ok(node)
    }
}

struct PrimaryHead;
impl Rule for PrimaryHead {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        Literal::starts_now(parser)
            || Array::starts_now(parser)
            || Hash::starts_now(parser)
            || VarRef::starts_now(parser)
            || BackRef::starts_now(parser)
            || parser.current_token().is(TokenKind::tFID)
            || BeginEndCompoundStmt::starts_now(parser)
            || ParenthesizedStmt::starts_now(parser)
            || GlobalConst::starts_now(parser)
            || NotExpr::starts_now(parser)
            || MethodCall::starts_now(parser)
            || SuperCall::starts_now(parser)
            || Lambda::starts_now(parser)
            || IfStmt::starts_now(parser)
            || UnlessStmt::starts_now(parser)
            || WhileStmt::starts_now(parser)
            || UntilStmt::starts_now(parser)
            || Case::starts_now(parser)
            || Class::starts_now(parser)
            || Module::starts_now(parser)
            || MethodDef::starts_now(parser)
            || KeywordCmd::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if Literal::starts_now(parser) {
            Literal::parse(parser)
        } else if Array::starts_now(parser) {
            Array::parse(parser)
        } else if Hash::starts_now(parser) {
            Hash::parse(parser)
        } else if VarRef::starts_now(parser) {
            VarRef::parse(parser)
        } else if parser.current_token().is(TokenKind::tFID) {
            let token = parser.take_token();
            todo!("build ??something?? from tFID {:?}", token)
        } else if BeginEndCompoundStmt::starts_now(parser) {
            BeginEndCompoundStmt::parse(parser)
        } else if ParenthesizedStmt::starts_now(parser) {
            ParenthesizedStmt::parse(parser)
        } else if GlobalConst::starts_now(parser) {
            GlobalConst::parse(parser)
        } else if NotExpr::starts_now(parser) {
            NotExpr::parse(parser)
        } else if MethodCall::starts_now(parser) {
            MethodCall::parse(parser)
        } else if SuperCall::starts_now(parser) {
            SuperCall::parse(parser)
        } else if Lambda::starts_now(parser) {
            Lambda::parse(parser)
        } else if IfStmt::starts_now(parser) {
            IfStmt::parse(parser)
        } else if UnlessStmt::starts_now(parser) {
            UnlessStmt::parse(parser)
        } else if WhileStmt::starts_now(parser) {
            WhileStmt::parse(parser)
        } else if UntilStmt::starts_now(parser) {
            UntilStmt::parse(parser)
        } else if Case::starts_now(parser) {
            Case::parse(parser)
        } else if ForLoop::starts_now(parser) {
            ForLoop::parse(parser)
        } else if Class::starts_now(parser) {
            Class::parse(parser)
        } else if Module::starts_now(parser) {
            Module::parse(parser)
        } else if MethodDef::starts_now(parser) {
            MethodDef::parse(parser)
        } else if KeywordCmd::starts_now(parser) {
            KeywordCmd::parse(parser)
        } else {
            unreachable!()
        }
    }
}

struct BeginEndCompoundStmt;
impl Rule for BeginEndCompoundStmt {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kBEGIN)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let _begin_t = parser.take_token();
        todo!()
    }
}

struct ParenthesizedStmt;
impl Rule for ParenthesizedStmt {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tLPAREN)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let _lparen_t = parser.take_token();
        todo!()
    }
}

struct GlobalConst;
impl Rule for GlobalConst {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tCOLON2)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let _colon2_t = parser.take_token();
        todo!()
    }
}

struct NotExpr;
impl Rule for NotExpr {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kNOT)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let _not_t = parser.take_token();
        todo!()
    }
}

struct MethodCall;
impl Rule for MethodCall {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        OperationT::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let _selector_t = parser.take_token();
        todo!()
    }
}

struct SuperCall;
impl Rule for SuperCall {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kSUPER)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let _super_t = parser.take_token();
        todo!()
    }
}

struct WhileStmt;
impl Rule for WhileStmt {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kWHILE)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let _while_t = parser.take_token();
        todo!()
    }
}

struct UntilStmt;
impl Rule for UntilStmt {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kUNTIL)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let _until_t = parser.take_token();
        todo!()
    }
}

struct KeywordCmd;
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

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
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
                    Args::parse(parser).expect("parse error")
                } else {
                    vec![]
                };

                let rparen_t = if lparen_t.is_some() {
                    Some(
                        parser
                            .expect_token(TokenKind::tRPAREN)
                            .expect("parse error"),
                    )
                } else {
                    None
                };

                Builder::yield_(keyword_t, lparen_t, args, rparen_t)
            }
            TokenKind::kDEFINED => {
                dbg!(parser.current_token());
                let lparen_t = parser.expect_token(TokenKind::tLPAREN).unwrap();
                let value = Expr::parse(parser).unwrap();
                let rparen_t = parser.expect_token(TokenKind::tRPAREN).unwrap();

                Builder::defined(keyword_t, Some(lparen_t), value, Some(rparen_t))
            }
            _ => unreachable!(),
        };

        Ok(node)
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

enum PrimaryTail {
    ConstAccess {
        colon2_t: Token,
        name_t: Token,
    },

    MethodCall {
        dot_t: Option<Token>,
        selector_t: Option<Token>,
        lparen_t: Option<Token>,
        args: Vec<Node>,
        rparen_t: Option<Token>,
        block_begin_t: Token,
        block_args: ArgsType,
        block_body: Option<Box<Node>>,
        block_end_t: Token,
    },

    IndexCall {
        lbrack_t: Token,
        indexes: Vec<Node>,
        rbrack_t: Token,
        block_begin_t: Token,
        block_args: ArgsType,
        block_body: Option<Box<Node>>,
        block_end_t: Token,
    },
}
impl Rule for PrimaryTail {
    type Output = Self;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        todo!()
    }
}
