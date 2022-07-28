use crate::{
    buffer::Buffer,
    builder::{Builder, Constructor},
    parser::{ParseError, ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl<C: Constructor> Parser<C> {
    pub(crate) fn try_arg(&mut self) -> ParseResult<Box<Node>> {
        try_arg0(self, 0)
    }
}

fn try_arg0<C: Constructor>(parser: &mut Parser<C>, min_bp: u8) -> ParseResult<Box<Node>> {
    let mut lhs: Box<Node> = try_arg_lhs(parser)?;

    loop {
        let op_t = try_binary_or_postfix_operator(parser);

        let op_t = if let Ok(tok) = op_t {
            tok
        } else {
            return Ok(lhs);
        };

        // handle postfix operators
        if is_postfix_operator(op_t) {
            // tDOT2 and tDOT3 can be postfix operators (endless ranges)
            if let Some((l_bp, _)) = op_t.kind.precedence() {
                if l_bp < min_bp {
                    break;
                }

                lhs = build_postfix_call(op_t, lhs);
                continue;
            }
        }

        // handle binary operators
        if let Some((l_bp, r_bp)) = op_t.kind.precedence() {
            if l_bp < min_bp {
                break;
            }

            if op_t.is(TokenKind::tEH) {
                // ternary operator
                let ternary = parser
                    .all_of("ternary operator")
                    .and(|| try_arg0(parser, 0))
                    .and(|| parser.expect_token(TokenKind::tCOLON))
                    .and(|| try_arg0(parser, r_bp))
                    .stop();

                let (mhs, colon_t, rhs) = match ternary {
                    Ok(values) => values,
                    Err(error) => {
                        return Err(ParseError::SeqError {
                            name: "ternary operator",
                            steps: vec![lhs.into(), op_t.into()],
                            error: Box::new(error),
                        })
                    }
                };
                lhs = Builder::<C>::ternary(lhs, op_t, mhs, colon_t, rhs);
            } else {
                // normal binary operator, like `+`
                let rhs = match try_arg0(parser, r_bp) {
                    Ok(node) => node,
                    Err(error) => {
                        return Err(ParseError::SeqError {
                            name: "rhs of binary operator",
                            steps: vec![lhs.into(), op_t.into()],
                            error: Box::new(error),
                        })
                    }
                };
                lhs = build_binary_call::<C>(lhs, op_t, rhs, parser.buffer());
            }
            continue;
        }

        // no postfix/binary operator found
        break;
    }

    Ok(lhs)
}

fn try_arg_head<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Box<Node>> {
    parser
        .one_of("arg head")
        .or_else(|| {
            let (lhs, eql_t, rhs) = parser
                .all_of("lhs tEQL arg_rhs")
                .and(|| parser.try_lhs())
                .and(|| parser.expect_token(TokenKind::tEQL))
                .and(|| try_arg_rhs(parser))
                .stop()?;

            todo!("{:?} {:?} {:?}", lhs, eql_t, rhs)
        })
        .or_else(|| {
            let (lhs, op_t, rhs) = parser
                .all_of("var_lhs tOP_ASGN arg_rhs")
                .and(|| parser.try_var_lhs())
                .and(|| parser.expect_token(TokenKind::tOP_ASGN))
                .and(|| try_arg_rhs(parser))
                .stop()?;

            todo!("{:?} {:?} {:?}", lhs, op_t, rhs)
        })
        .or_else(|| {
            let (expr, lbrack_t, args, rparen_t, op_t, rhs) = parser
                .all_of("primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN arg_rhs")
                .and(|| parser.try_primary_value())
                .and(|| parser.expect_token(TokenKind::tLBRACK))
                .and(|| parser.try_opt_call_args())
                .and(|| parser.try_rbracket())
                .and(|| parser.expect_token(TokenKind::tOP_ASGN))
                .and(|| try_arg_rhs(parser))
                .stop()?;
            todo!(
                "{:?} {:?} {:?} {:?} {:?} {:?}",
                expr,
                lbrack_t,
                args,
                rparen_t,
                op_t,
                rhs
            )
        })
        .or_else(|| {
            let (expr, call_op_t, mid_t, op_t, rhs) = parser
                .all_of("primary_value call_op2 tIDENTIFIER tOP_ASGN arg_rhs")
                .and(|| parser.try_primary_value())
                .and(|| parser.try_call_op2())
                .and(|| parser.expect_token(TokenKind::tIDENTIFIER))
                .and(|| parser.expect_token(TokenKind::tOP_ASGN))
                .and(|| try_arg_rhs(parser))
                .stop()?;

            todo!(
                "{:?} {:?} {:?} {:?} {:?}",
                expr,
                call_op_t,
                mid_t,
                op_t,
                rhs
            )
        })
        .or_else(|| {
            let (expr, call_op_t, const_t, op_t, rhs) = parser
                .all_of("primary_value call_op2 tCONSTANT tOP_ASGN arg_rhs")
                .and(|| parser.try_primary_value())
                .and(|| parser.try_call_op2())
                .and(|| parser.expect_token(TokenKind::tCONSTANT))
                .and(|| parser.expect_token(TokenKind::tOP_ASGN))
                .and(|| try_arg_rhs(parser))
                .stop()?;

            todo!(
                "{:?} {:?} {:?} {:?} {:?}",
                expr,
                call_op_t,
                const_t,
                op_t,
                rhs
            )
        })
        .or_else(|| {
            let (colon_t, const_t, op_t, rhs) = parser
                .all_of("tCOLON3 tCONSTANT tOP_ASGN arg_rhs")
                .and(|| parser.expect_token(TokenKind::tCOLON2))
                .and(|| parser.expect_token(TokenKind::tCONSTANT))
                .and(|| parser.expect_token(TokenKind::tOP_ASGN))
                .and(|| try_arg_rhs(parser))
                .stop()?;

            todo!("{:?} {:?} {:?} {:?}", colon_t, const_t, op_t, rhs)
        })
        .or_else(|| {
            let (lhs, op_t, rhs) = parser
                .all_of("backref tOP_ASGN arg_rhs")
                .and(|| parser.try_back_ref())
                .and(|| parser.expect_token(TokenKind::tOP_ASGN))
                .and(|| try_arg_rhs(parser))
                .stop()?;

            todo!("{:?} {:?} {:?}", lhs, op_t, rhs)
        })
        .or_else(|| {
            let (minus_t, lhs, op_t, rhs) = parser
                .all_of("tUMINUS_NUM simple_numeric tPOW arg")
                .and(|| parser.expect_token(TokenKind::tUMINUS_NUM))
                .and(|| parser.try_simple_numeric())
                .and(|| parser.expect_token(TokenKind::tPOW))
                .and(|| parser.try_arg())
                .stop()?;

            todo!("{:?} {:?} {:?} {:?}", minus_t, lhs, op_t, rhs)
        })
        .or_else(|| {
            let (def, args, eql_t, body, rescue_t, rescue_body) = parser
                .all_of("defn_head f_opt_paren_args tEQL arg kRESCUE_MOD arg")
                .and(|| parser.try_defn_head())
                .and(|| parser.try_f_opt_paren_args())
                .and(|| parser.expect_token(TokenKind::tEQL))
                .and(|| parser.try_arg())
                .and(|| parser.expect_token(TokenKind::kRESCUE_MOD))
                .and(|| parser.try_arg())
                .stop()?;

            todo!(
                "{:?} {:?} {:?} {:?} {:?} {:?}",
                def,
                args,
                eql_t,
                body,
                rescue_t,
                rescue_body
            )
        })
        .or_else(|| {
            let (def, args, eql_t, body) = parser
                .all_of("defn_head f_opt_paren_args tEQL arg")
                .and(|| parser.try_defn_head())
                .and(|| parser.try_f_opt_paren_args())
                .and(|| parser.expect_token(TokenKind::tEQL))
                .and(|| parser.try_arg())
                .stop()?;

            todo!("{:?} {:?} {:?} {:?}", def, args, eql_t, body)
        })
        .or_else(|| {
            let (defs, args, eql_t, body, rescue_t, rescue_body) = parser
                .all_of("defs_head f_opt_paren_args tEQL arg kRESCUE_MOD arg")
                .and(|| parser.try_defs_head())
                .and(|| parser.try_f_opt_paren_args())
                .and(|| parser.expect_token(TokenKind::tEQL))
                .and(|| parser.try_arg())
                .and(|| parser.expect_token(TokenKind::kRESCUE_MOD))
                .and(|| parser.try_arg())
                .stop()?;

            todo!(
                "{:?} {:?} {:?} {:?} {:?} {:?}",
                defs,
                args,
                eql_t,
                body,
                rescue_t,
                rescue_body
            )
        })
        .or_else(|| {
            let (defs, args, eql_t, body) = parser
                .all_of("defs_head f_opt_paren_args tEQL arg")
                .and(|| parser.try_defs_head())
                .and(|| parser.try_f_opt_paren_args())
                .and(|| parser.expect_token(TokenKind::tEQL))
                .and(|| parser.try_arg())
                .stop()?;

            todo!("{:?} {:?} {:?} {:?}", defs, args, eql_t, body,)
        })
        .or_else(|| parser.try_primary())
        .stop()
}

fn try_arg_rhs<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Box<Node>> {
    let lhs = parser.try_arg()?;
    if parser.current_token().is(TokenKind::kRESCUE) {
        let rescue_t = parser.current_token();
        parser.skip_token();

        match parser.try_arg() {
            Ok(rhs) => todo!("{:?} {:?} {:?}", lhs, rescue_t, rhs),
            Err(error) => Err(ParseError::SeqError {
                name: "arg rescue arg",
                steps: vec![lhs.into(), rescue_t.into()],
                error: Box::new(error),
            }),
        }
    } else {
        Ok(lhs)
    }
}

fn try_arg_prefix_operator<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Token> {
    parser
        .one_of("arg prefix operator")
        .or_else(|| parser.try_token(TokenKind::tBDOT2))
        .or_else(|| parser.try_token(TokenKind::tBDOT3))
        .or_else(|| parser.try_token(TokenKind::tUPLUS))
        .or_else(|| parser.try_token(TokenKind::tUMINUS))
        .or_else(|| parser.try_token(TokenKind::tBANG))
        .or_else(|| parser.try_token(TokenKind::tTILDE))
        .or_else(|| parser.try_token(TokenKind::kDEFINED))
        .stop()
}

fn try_arg_lhs<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Box<Node>> {
    parser
        .one_of("arg lhs")
        .or_else(|| {
            let op_t = try_arg_prefix_operator(parser)?;
            let (_, r_bp) = op_t.kind.precedence().expect("bug");
            let rhs = try_arg0(parser, r_bp)?;

            todo!("unary {:?} {:?}", op_t, rhs);
        })
        .or_else(|| try_arg_head(parser))
        .stop()
}

fn try_binary_or_postfix_operator<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Token> {
    parser
        .one_of("infix/binary operator")
        .or_else(|| parser.expect_token(TokenKind::tDOT2))
        .or_else(|| parser.expect_token(TokenKind::tDOT3))
        .or_else(|| parser.expect_token(TokenKind::tPLUS))
        .or_else(|| parser.expect_token(TokenKind::tMINUS))
        .or_else(|| parser.expect_token(TokenKind::tSTAR))
        .or_else(|| parser.expect_token(TokenKind::tDIVIDE))
        .or_else(|| parser.expect_token(TokenKind::tPERCENT))
        .or_else(|| parser.expect_token(TokenKind::tPOW))
        .or_else(|| parser.expect_token(TokenKind::tPIPE))
        .or_else(|| parser.expect_token(TokenKind::tCARET))
        .or_else(|| parser.expect_token(TokenKind::tAMPER))
        .or_else(|| parser.expect_token(TokenKind::tCMP))
        .or_else(|| parser.expect_token(TokenKind::tEQ))
        .or_else(|| parser.expect_token(TokenKind::tEQQ))
        .or_else(|| parser.expect_token(TokenKind::tNEQ))
        .or_else(|| parser.expect_token(TokenKind::tMATCH))
        .or_else(|| parser.expect_token(TokenKind::tNMATCH))
        .or_else(|| parser.expect_token(TokenKind::tLSHFT))
        .or_else(|| parser.expect_token(TokenKind::tRSHFT))
        .or_else(|| parser.expect_token(TokenKind::tANDOP))
        .or_else(|| parser.expect_token(TokenKind::tOROP))
        .or_else(|| parser.expect_token(TokenKind::tEH))
        .or_else(|| parser.expect_token(TokenKind::tGT))
        .or_else(|| parser.expect_token(TokenKind::tLT))
        .or_else(|| parser.expect_token(TokenKind::tGEQ))
        .or_else(|| parser.expect_token(TokenKind::tLEQ))
        .stop()
}

fn is_postfix_operator(op_t: Token) -> bool {
    matches!(op_t.kind, TokenKind::tDOT2 | TokenKind::tDOT3)
}

fn build_postfix_call(op_t: Token, _lhs: Box<Node>) -> Box<Node> {
    match op_t.kind {
        TokenKind::tDOT2 => panic!("range inclusive"),
        TokenKind::tDOT3 => panic!("range exclusive"),

        _ => unreachable!("not a postfix operator {:?}", op_t),
    }
}

fn build_binary_call<C: Constructor>(
    lhs: Box<Node>,
    op_t: Token,
    rhs: Box<Node>,
    buffer: &Buffer,
) -> Box<Node> {
    match op_t.kind {
        TokenKind::tDOT2 => panic!("range inclusive"),
        TokenKind::tDOT3 => panic!("range exclusive"),

        TokenKind::tPLUS
        | TokenKind::tMINUS
        | TokenKind::tSTAR
        | TokenKind::tDIVIDE
        | TokenKind::tPERCENT
        | TokenKind::tPOW
        | TokenKind::tPIPE
        | TokenKind::tCARET
        | TokenKind::tAMPER
        | TokenKind::tCMP
        | TokenKind::tEQ
        | TokenKind::tEQQ
        | TokenKind::tNEQ
        | TokenKind::tNMATCH
        | TokenKind::tLSHFT
        | TokenKind::tRSHFT
        | TokenKind::tGT
        | TokenKind::tLT
        | TokenKind::tGEQ
        | TokenKind::tLEQ => Builder::<C>::binary_op(lhs, op_t, rhs, buffer),

        TokenKind::tMATCH => Builder::<C>::match_op(lhs, op_t, rhs),

        TokenKind::tANDOP | TokenKind::tOROP => Builder::<C>::logical_op(lhs, op_t, rhs),

        _ => unreachable!("not a binary operator {:?}", op_t.kind),
    }
}

#[test]
fn test_arg() {
    use crate::testing::assert_parses;

    assert_parses!(
        try_arg,
        b"1 + 2 * 3",
        r#"
s(:send,
  s(:int, "1"), "+",
  s(:send,
    s(:int, "2"), "*",
    s(:int, "3")))
        "#
    );
}
