use crate::{
    builder::Builder,
    parser::{macros::all_of, ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_expr(&mut self) -> ParseResult<Box<Node>> {
        let mut lhs = parse_expr_head(self)?;
        while let Some((op_t, rhs)) = try_expr_tail(self)? {
            lhs = Builder::logical_op(lhs, op_t, rhs)
        }
        Ok(lhs)
    }
}

fn parse_expr_head(parser: &mut Parser) -> ParseResult<Box<Node>> {
    parser
        .one_of("expression")
        .or_else(|| parser.parse_command_call())
        .or_else(|| parse_not_expr(parser))
        .or_else(|| parse_bang_command_call(parser))
        .or_else(|| parse_arg_assoc_p_expr_body(parser))
        .or_else(|| parse_arg_in_p_expr_body(parser))
        .or_else(|| parser.parse_arg())
        .stop()
}
fn parse_not_expr(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (not_t, _opt_nl, expr) = all_of!(
        "not expr",
        parser.try_token(TokenKind::kNOT),
        parser.try_opt_nl(),
        parser.parse_expr(),
    )?;

    Ok(Builder::not_op(not_t, None, Some(expr), None))
}
fn parse_bang_command_call(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (bang_t, command_call) = all_of!(
        "! command_call",
        parser.try_token(TokenKind::tBANG),
        parser.parse_command_call(),
    )?;

    Ok(Builder::not_op(bang_t, None, Some(command_call), None))
}
fn parse_arg_assoc_p_expr_body(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (arg, assoc_t, p_top_expr_body) = all_of!(
        "arg => pattern",
        parser.parse_arg(),
        parser.try_token(TokenKind::tASSOC),
        parser.parse_p_top_expr_body(),
    )?;

    Ok(Builder::match_pattern(arg, assoc_t, p_top_expr_body))
}
fn parse_arg_in_p_expr_body(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (arg, in_t, p_top_expr_body) = all_of!(
        "arg in pattern",
        parser.parse_arg(),
        parser.try_token(TokenKind::kIN),
        parser.parse_p_top_expr_body(),
    )?;

    Ok(Builder::match_pattern_p(arg, in_t, p_top_expr_body))
}

fn try_expr_tail(parser: &mut Parser) -> ParseResult<Option<(Token, Box<Node>)>> {
    let expr_tail = parser
        .one_of("[and/or] expr")
        .or_else(|| {
            all_of!(
                "and expr",
                parser.try_token(TokenKind::kAND),
                parser.parse_expr(),
            )
        })
        .or_else(|| {
            all_of!(
                "or expr",
                parser.try_token(TokenKind::kOR),
                parser.parse_expr(),
            )
        })
        .stop();

    match expr_tail {
        Ok(data) => Ok(Some(data)),
        Err(error) => {
            match error.strip_lookaheads() {
                Some(error) => Err(error),
                None => {
                    // no match
                    Ok(None)
                }
            }
        }
    }
}
