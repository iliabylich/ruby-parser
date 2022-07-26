use crate::{
    builder::{Builder, Constructor},
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    transactions::ParseResultApi,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_expr(&mut self) -> ParseResult<Box<Node>> {
        let mut lhs = try_expr_head(self)?;
        while let Some((op_t, rhs)) = try_expr_tail(self)? {
            lhs = Builder::<C>::logical_op(lhs, op_t, rhs)
        }
        Ok(lhs)
    }
}

fn try_expr_head<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Box<Node>> {
    parser
        .one_of("expression")
        .or_else(|| parser.try_command_call())
        .or_else(|| try_not_expr(parser))
        .or_else(|| try_bang_command_call(parser))
        .or_else(|| try_arg_assoc_p_expr_body(parser))
        .or_else(|| try_arg_in_p_expr_body(parser))
        .or_else(|| parser.try_arg())
        .stop()
}
fn try_not_expr<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Box<Node>> {
    let (not_t, _opt_nl, expr) = parser
        .all_of("not expr")
        .and(|| parser.try_token(TokenKind::kNOT))
        .and(|| parser.try_opt_nl())
        .and(|| parser.try_expr())
        .stop()?;

    Ok(Builder::<C>::not_op(not_t, None, Some(expr), None))
}
fn try_bang_command_call<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Box<Node>> {
    let (bang_t, command_call) = parser
        .all_of("! command_call")
        .and(|| parser.try_token(TokenKind::tBANG))
        .and(|| parser.try_command_call())
        .stop()?;

    Ok(Builder::<C>::not_op(bang_t, None, Some(command_call), None))
}
fn try_arg_assoc_p_expr_body<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Box<Node>> {
    let (arg, assoc_t, p_top_expr_body) = parser
        .all_of("arg => pattern")
        .and(|| parser.try_arg())
        .and(|| parser.try_token(TokenKind::tASSOC))
        .and(|| parser.try_p_top_expr_body())
        .stop()?;

    Ok(Builder::<C>::match_pattern(arg, assoc_t, p_top_expr_body))
}
fn try_arg_in_p_expr_body<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Box<Node>> {
    let (arg, in_t, p_top_expr_body) = parser
        .all_of("arg in pattern")
        .and(|| parser.try_arg())
        .and(|| parser.try_token(TokenKind::kIN))
        .and(|| parser.try_p_top_expr_body())
        .stop()?;

    Ok(Builder::<C>::match_pattern_p(arg, in_t, p_top_expr_body))
}

fn try_expr_tail<C: Constructor>(
    parser: &mut Parser<C>,
) -> ParseResult<Option<(Token, Box<Node>)>> {
    parser
        .one_of("[and/or] expr")
        .or_else(|| {
            parser
                .all_of("and expr")
                .and(|| parser.try_token(TokenKind::kAND))
                .and(|| parser.try_expr())
                .stop()
        })
        .or_else(|| {
            parser
                .all_of("or expr")
                .and(|| parser.try_token(TokenKind::kOR))
                .and(|| parser.try_expr())
                .stop()
        })
        .stop()
        .ignore_lookaheads()
}
