use crate::{
    builder::Builder,
    parser::{ParseResult, Parser},
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
    let (not_t, _opt_nl, expr) = parser
        .all_of("not expr")
        .and(|| parser.try_token(TokenKind::kNOT))
        .and(|| parser.try_opt_nl())
        .and(|| parser.parse_expr())
        .stop()?;

    Ok(Builder::not_op(not_t, None, Some(expr), None))
}
fn parse_bang_command_call(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (bang_t, command_call) = parser
        .all_of("! command_call")
        .and(|| parser.try_token(TokenKind::tBANG))
        .and(|| parser.parse_command_call())
        .stop()?;

    Ok(Builder::not_op(bang_t, None, Some(command_call), None))
}
fn parse_arg_assoc_p_expr_body(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (arg, assoc_t, p_top_expr_body) = parser
        .all_of("arg => pattern")
        .and(|| parser.parse_arg())
        .and(|| parser.try_token(TokenKind::tASSOC))
        .and(|| parser.parse_p_top_expr_body())
        .stop()?;

    Ok(Builder::match_pattern(arg, assoc_t, p_top_expr_body))
}
fn parse_arg_in_p_expr_body(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (arg, in_t, p_top_expr_body) = parser
        .all_of("arg in pattern")
        .and(|| parser.parse_arg())
        .and(|| parser.try_token(TokenKind::kIN))
        .and(|| parser.parse_p_top_expr_body())
        .stop()?;

    Ok(Builder::match_pattern_p(arg, in_t, p_top_expr_body))
}

fn try_expr_tail(parser: &mut Parser) -> ParseResult<Option<(Token, Box<Node>)>> {
    let expr_tail = parser
        .one_of("[and/or] expr")
        .or_else(|| {
            parser
                .all_of("and expr")
                .and(|| parser.try_token(TokenKind::kAND))
                .and(|| parser.parse_expr())
                .stop()
        })
        .or_else(|| {
            parser
                .all_of("or expr")
                .and(|| parser.try_token(TokenKind::kOR))
                .and(|| parser.parse_expr())
                .stop()
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
