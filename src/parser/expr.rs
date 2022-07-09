use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::{Token, TokenKind},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_expr(&mut self) -> Option<Box<Node>> {
        let mut lhs = try_expr_head(self)?;
        while let Some((op_t, rhs)) = try_expr_tail(self) {
            lhs = Builder::<C>::logical_op(lhs, op_t, rhs)
        }
        Some(lhs)
    }
}

fn try_expr_head<C: Constructor>(parser: &mut Parser<C>) -> Option<Box<Node>> {
    None.or_else(|| parser.try_command_call())
        .or_else(|| try_not_expr(parser))
        .or_else(|| try_bang_command_call(parser))
        .or_else(|| try_arg_assoc_p_expr_body(parser))
        .or_else(|| try_arg_in_p_expr_body(parser))
        .or_else(|| parser.try_arg())
}
fn try_not_expr<C: Constructor>(parser: &mut Parser<C>) -> Option<Box<Node>> {
    let checkpoint = parser.new_checkpoint();
    let not_t = parser.try_token(TokenKind::kNOT)?;
    let _ = parser.try_opt_nl();
    if let Some(expr) = parser.try_expr() {
        return Some(Builder::<C>::not_op(not_t, None, Some(expr), None));
    }
    parser.restore_checkpoint(checkpoint);
    None
}
fn try_bang_command_call<C: Constructor>(parser: &mut Parser<C>) -> Option<Box<Node>> {
    let checkpoint = parser.new_checkpoint();
    let bang_t = parser.try_token(TokenKind::tBANG)?;
    if let Some(command_call) = parser.try_command_call() {
        return Some(Builder::<C>::not_op(bang_t, None, Some(command_call), None));
    }
    parser.restore_checkpoint(checkpoint);
    None
}
fn try_arg_assoc_p_expr_body<C: Constructor>(parser: &mut Parser<C>) -> Option<Box<Node>> {
    let checkpoint = parser.new_checkpoint();
    let arg = parser.try_arg()?;
    if let Some(assoc_t) = parser.try_token(TokenKind::tASSOC) {
        if let Some(p_top_expr_body) = parser.try_p_top_expr_body() {
            return Some(Builder::<C>::match_pattern(arg, assoc_t, p_top_expr_body));
        } else {
            panic!(
                "arg kIN expected p_top_expr_body, got {:?}",
                parser.current_token()
            )
        }
    }
    parser.restore_checkpoint(checkpoint);
    None
}
fn try_arg_in_p_expr_body<C: Constructor>(parser: &mut Parser<C>) -> Option<Box<Node>> {
    let checkpoint = parser.new_checkpoint();
    let arg = parser.try_arg()?;
    if let Some(in_t) = parser.try_token(TokenKind::kIN) {
        if let Some(p_top_expr_body) = parser.try_p_top_expr_body() {
            return Some(Builder::<C>::match_pattern_p(arg, in_t, p_top_expr_body));
        } else {
            panic!(
                "arg kIN expected p_top_expr_body, got {:?}",
                parser.current_token()
            )
        }
    }
    parser.restore_checkpoint(checkpoint);
    None
}

fn try_expr_tail<C: Constructor>(parser: &mut Parser<C>) -> Option<(Token, Box<Node>)> {
    let op_t = None
        .or_else(|| parser.try_token(TokenKind::kAND))
        .or_else(|| parser.try_token(TokenKind::kOR))?;
    if let Some(rhs) = parser.try_expr() {
        Some((op_t, rhs))
    } else {
        panic!("expected RHS of the binary expression")
    }
}
