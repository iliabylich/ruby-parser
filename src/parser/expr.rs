use crate::{
    builder::{Builder, Constructor},
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
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
        .unwrap()
}
fn try_not_expr<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Box<Node>> {
    let not_t = parser.try_token(TokenKind::kNOT)?;
    let _ = parser.try_opt_nl();
    let expr = parser.try_expr()?;
    Ok(Builder::<C>::not_op(not_t, None, Some(expr), None))
}
fn try_bang_command_call<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Box<Node>> {
    let bang_t = parser.try_token(TokenKind::tBANG)?;
    let command_call = parser.try_command_call()?;
    Ok(Builder::<C>::not_op(bang_t, None, Some(command_call), None))
}
fn try_arg_assoc_p_expr_body<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Box<Node>> {
    let arg = parser.try_arg()?;
    let assoc_t = parser.try_token(TokenKind::tASSOC)?;
    let p_top_expr_body = parser.try_p_top_expr_body()?;
    Ok(Builder::<C>::match_pattern(arg, assoc_t, p_top_expr_body))
}
fn try_arg_in_p_expr_body<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Box<Node>> {
    let arg = parser.try_arg()?;
    let in_t = parser.try_token(TokenKind::kIN)?;
    let p_top_expr_body = parser.try_p_top_expr_body()?;
    Ok(Builder::<C>::match_pattern_p(arg, in_t, p_top_expr_body))
}

fn try_expr_tail<C: Constructor>(
    parser: &mut Parser<C>,
) -> ParseResult<Option<(Token, Box<Node>)>> {
    let op_t = parser
        .one_of("expression continuation")
        .or_else(|| parser.try_token(TokenKind::kAND))
        .or_else(|| parser.try_token(TokenKind::kOR))
        .unwrap()
        .ok();
    let op_t = if let Some(op_t) = op_t {
        op_t
    } else {
        return Ok(None);
    };

    let rhs = parser.try_expr()?;
    Ok(Some((op_t, rhs)))
}
