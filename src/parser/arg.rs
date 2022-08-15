use crate::{
    buffer::Buffer,
    builder::{Builder, KeywordCmd},
    parser::{macros::all_of, ParseError, ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_arg(&mut self) -> ParseResult<Box<Node>> {
        parse_arg0(self, 0)
    }
}

fn parse_arg0(parser: &mut Parser, min_bp: u8) -> ParseResult<Box<Node>> {
    let mut lhs: Box<Node> = parse_arg_lhs(parser)?;

    loop {
        let op_t = parse_binary_or_postfix_operator(parser);

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
                let ternary = all_of!(
                    "ternary operator",
                    parse_arg0(parser, 0),
                    parser.expect_token(TokenKind::tCOLON),
                    parse_arg0(parser, r_bp),
                );

                let (mhs, colon_t, rhs) = match ternary {
                    Ok(values) => values,
                    Err(error) => {
                        return Err(ParseError::seq_error(
                            "ternary operator",
                            (lhs, op_t),
                            error,
                        ))
                    }
                };
                lhs = Builder::ternary(lhs, op_t, mhs, colon_t, rhs);
            } else {
                // normal binary operator, like `+`
                let rhs = match parse_arg0(parser, r_bp) {
                    Ok(node) => node,
                    Err(error) => {
                        return Err(ParseError::seq_error(
                            "rhs of binary operator",
                            (lhs, op_t),
                            error,
                        ))
                    }
                };
                lhs = build_binary_call(lhs, op_t, rhs, parser.buffer());
            }
            continue;
        }

        // no postfix/binary operator found
        break;
    }

    Ok(lhs)
}

fn parse_arg_head(parser: &mut Parser) -> ParseResult<Box<Node>> {
    parser
        .one_of("arg head")
        .or_else(|| {
            let (lhs, eql_t, rhs) = all_of!(
                "lhs tEQL arg_rhs",
                parser.parse_lhs(),
                parser.expect_token(TokenKind::tEQL),
                parse_arg_rhs(parser),
            )?;

            Ok(Builder::assign(lhs, eql_t, rhs))
        })
        .or_else(|| {
            let (lhs, op_t, rhs) = all_of!(
                "var_lhs tOP_ASGN arg_rhs",
                parser.parse_var_lhs(),
                parser.expect_token(TokenKind::tOP_ASGN),
                parse_arg_rhs(parser),
            )?;

            Ok(Builder::op_assign(lhs, op_t, rhs, parser.buffer()))
        })
        .or_else(|| {
            let (expr, lbrack_t, args, rbrack_t, op_t, rhs) = all_of!(
                "primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN arg_rhs",
                parser.parse_primary_value(),
                parser.expect_token(TokenKind::tLBRACK),
                parser.parse_opt_call_args(),
                parser.parse_rbracket(),
                parser.expect_token(TokenKind::tOP_ASGN),
                parse_arg_rhs(parser),
            )?;

            Ok(Builder::op_assign(
                Builder::index(expr, lbrack_t, args, rbrack_t),
                op_t,
                rhs,
                parser.buffer(),
            ))
        })
        .or_else(|| {
            let (expr, call_op_t, mid_t, op_t, rhs) = all_of!(
                "primary_value call_op2 tIDENTIFIER tOP_ASGN arg_rhs",
                parser.parse_primary_value(),
                parser.parse_call_op2(),
                parser.expect_token(TokenKind::tIDENTIFIER),
                parser.expect_token(TokenKind::tOP_ASGN),
                parse_arg_rhs(parser),
            )?;

            Ok(Builder::op_assign(
                Builder::call_method(
                    Some(expr),
                    Some(call_op_t),
                    Some(mid_t),
                    None,
                    vec![],
                    None,
                    parser.buffer(),
                ),
                op_t,
                rhs,
                parser.buffer(),
            ))
        })
        .or_else(|| {
            let (expr, call_op_t, const_t, op_t, rhs) = all_of!(
                "primary_value call_op2 tCONSTANT tOP_ASGN arg_rhs",
                parser.parse_primary_value(),
                parser.parse_call_op2(),
                parser.expect_token(TokenKind::tCONSTANT),
                parser.expect_token(TokenKind::tOP_ASGN),
                parse_arg_rhs(parser),
            )?;

            Ok(Builder::op_assign(
                Builder::call_method(
                    Some(expr),
                    Some(call_op_t),
                    Some(const_t),
                    None,
                    vec![],
                    None,
                    parser.buffer(),
                ),
                op_t,
                rhs,
                parser.buffer(),
            ))
        })
        .or_else(|| {
            let (colon2_t, const_t, op_t, rhs) = all_of!(
                "tCOLON3 tCONSTANT tOP_ASGN arg_rhs",
                parser.expect_token(TokenKind::tCOLON2),
                parser.expect_token(TokenKind::tCONSTANT),
                parser.expect_token(TokenKind::tOP_ASGN),
                parse_arg_rhs(parser),
            )?;

            Ok(Builder::op_assign(
                Builder::const_op_assignable(Builder::const_global(
                    colon2_t,
                    const_t,
                    parser.buffer(),
                )),
                op_t,
                rhs,
                parser.buffer(),
            ))
        })
        .or_else(|| {
            let (lhs, op_t, rhs) = all_of!(
                "backref tOP_ASGN arg_rhs",
                parser.try_back_ref(),
                parser.expect_token(TokenKind::tOP_ASGN),
                parse_arg_rhs(parser),
            )?;

            Ok(Builder::op_assign(lhs, op_t, rhs, parser.buffer()))
        })
        .or_else(|| {
            let (minus_t, lhs, op_t, rhs) = all_of!(
                "tUMINUS_NUM simple_numeric tPOW arg",
                parser.expect_token(TokenKind::tUMINUS_NUM),
                parser.try_simple_numeric(),
                parser.expect_token(TokenKind::tPOW),
                parser.parse_arg(),
            )?;

            Ok(Builder::unary_op(
                minus_t,
                Builder::binary_op(lhs, op_t, rhs, parser.buffer()),
                parser.buffer(),
            ))
        })
        .or_else(|| {
            let ((def_t, name_t), args, eql_t, body, rescue_t, rescue_body) = all_of!(
                "defn_head f_opt_paren_args tEQL arg kRESCUE_MOD arg",
                parser.parse_defn_head(),
                parser.try_f_opt_paren_args(),
                parser.expect_token(TokenKind::tEQL),
                parser.parse_arg(),
                parser.expect_token(TokenKind::kRESCUE_MOD),
                parser.parse_arg(),
            )?;

            // self.validate_endless_method_name(&name_t);

            let rescue_body = Builder::rescue_body(rescue_t, vec![], None, None, Some(rescue_body));

            let method_body = Builder::begin_body(Some(body), vec![*rescue_body], None, None);

            Ok(Builder::def_endless_method(
                def_t,
                name_t,
                args,
                eql_t,
                Some(method_body),
                parser.buffer(),
            ))
        })
        .or_else(|| {
            let ((def_t, name_t), args, eql_t, body) = all_of!(
                "defn_head f_opt_paren_args tEQL arg",
                parser.parse_defn_head(),
                parser.try_f_opt_paren_args(),
                parser.expect_token(TokenKind::tEQL),
                parser.parse_arg(),
            )?;

            Ok(Builder::def_endless_method(
                def_t,
                name_t,
                args,
                eql_t,
                Some(body),
                parser.buffer(),
            ))
        })
        .or_else(|| {
            let ((def_t, definee, dot_t, name_t), args, eql_t, body, rescue_t, rescue_body) =
                all_of!(
                    "defs_head f_opt_paren_args tEQL arg kRESCUE_MOD arg",
                    parser.parse_defs_head(),
                    parser.try_f_opt_paren_args(),
                    parser.expect_token(TokenKind::tEQL),
                    parser.parse_arg(),
                    parser.expect_token(TokenKind::kRESCUE_MOD),
                    parser.parse_arg(),
                )?;

            let rescue_body = Builder::rescue_body(rescue_t, vec![], None, None, Some(rescue_body));

            let method_body = Builder::begin_body(Some(body), vec![*rescue_body], None, None);

            Ok(Builder::def_endless_singleton(
                def_t,
                definee,
                dot_t,
                name_t,
                args,
                eql_t,
                Some(method_body),
                parser.buffer(),
            ))
        })
        .or_else(|| {
            let ((def_t, definee, dot_t, name_t), args, eql_t, body) = all_of!(
                "defs_head f_opt_paren_args tEQL arg",
                parser.parse_defs_head(),
                parser.try_f_opt_paren_args(),
                parser.expect_token(TokenKind::tEQL),
                parser.parse_arg(),
            )?;

            Ok(Builder::def_endless_singleton(
                def_t,
                definee,
                dot_t,
                name_t,
                args,
                eql_t,
                Some(body),
                parser.buffer(),
            ))
        })
        .or_else(|| parser.parse_primary())
        .stop()
}

fn parse_arg_rhs(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let lhs = parser.parse_arg()?;
    if parser.current_token().is(TokenKind::kRESCUE) {
        let rescue_t = parser.current_token();
        parser.skip_token();

        match parser.parse_arg() {
            Ok(rhs) => {
                // self.value_expr(&lhs);
                let rescue_body = Builder::rescue_body(rescue_t, vec![], None, None, Some(rhs));
                Ok(Builder::begin_body(
                    Some(lhs),
                    vec![*rescue_body],
                    None,
                    None,
                ))
            }
            Err(error) => Err(ParseError::seq_error(
                "arg rescue arg",
                (lhs, rescue_t),
                error,
            )),
        }
    } else {
        Ok(lhs)
    }
}

fn parse_arg_prefix_operator(parser: &mut Parser) -> ParseResult<Token> {
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

fn parse_arg_lhs(parser: &mut Parser) -> ParseResult<Box<Node>> {
    parser
        .one_of("arg lhs")
        .or_else(|| {
            let op_t = parse_arg_prefix_operator(parser)?;
            let (_, r_bp) = op_t.kind.precedence().expect("bug");
            let rhs = parse_arg0(parser, r_bp)?;

            match op_t.kind {
                TokenKind::tBDOT2 => {
                    // self.value_expr(&rhs);
                    Ok(Builder::range_inclusive(None, op_t, Some(rhs)))
                }
                TokenKind::tBDOT3 => {
                    // self.value_expr(&rhs);
                    Ok(Builder::range_exclusive(None, op_t, Some(rhs)))
                }
                TokenKind::tUMINUS | TokenKind::tUPLUS | TokenKind::tTILDE => {
                    Ok(Builder::unary_op(op_t, rhs, parser.buffer()))
                }
                TokenKind::tBANG => Ok(Builder::not_op(op_t, None, Some(rhs), None)),
                TokenKind::kDEFINED => Ok(Builder::keyword_cmd(
                    KeywordCmd::Defined,
                    op_t,
                    None,
                    vec![*rhs],
                    None,
                )),
                _ => {
                    unreachable!("bug")
                }
            }
        })
        .or_else(|| parse_arg_head(parser))
        .stop()
}

fn parse_binary_or_postfix_operator(parser: &mut Parser) -> ParseResult<Token> {
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

fn build_binary_call(lhs: Box<Node>, op_t: Token, rhs: Box<Node>, buffer: &Buffer) -> Box<Node> {
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
        | TokenKind::tLEQ => Builder::binary_op(lhs, op_t, rhs, buffer),

        TokenKind::tMATCH => Builder::match_op(lhs, op_t, rhs),

        TokenKind::tANDOP | TokenKind::tOROP => Builder::logical_op(lhs, op_t, rhs),

        _ => unreachable!("not a binary operator {:?}", op_t.kind),
    }
}

#[test]
fn test_arg() {
    use crate::testing::assert_parses;

    assert_parses!(
        parse_arg,
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
