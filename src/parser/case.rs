use crate::{
    builder::Builder,
    parser::{
        macros::{all_of, at_least_once, one_of},
        ParseResult, Parser,
    },
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_case(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "case expr",
            checkpoint = self.new_checkpoint(),
            parse_case_when1(self),
            parse_case_when2(self),
            parse_case_in(self),
        )
    }
}

fn parse_case_when1(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (case_t, expr, _terms, when_bodies, opt_else, end_t) = all_of!(
        "k_case expr_value opt_terms case_body k_end",
        parse_k_case(parser),
        parser.parse_expr_value(),
        parser.parse_opt_terms(),
        parse_when_bodies(parser),
        parser.try_opt_else(),
        parser.parse_k_end(),
    )?;

    let (else_t, else_body) = opt_else
        .map(|(else_t, else_body)| (Some(else_t), else_body))
        .unwrap_or_else(|| (None, None));

    Ok(Builder::case(
        case_t,
        Some(expr),
        when_bodies,
        else_t,
        else_body,
        end_t,
    ))
}

fn parse_case_when2(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (case_t, _opt_terms, when_bodies, opt_else, end_t) = all_of!(
        "k_case opt_terms case_body k_end",
        parse_k_case(parser),
        parser.parse_opt_terms(),
        parse_when_bodies(parser),
        parser.try_opt_else(),
        parser.parse_k_end(),
    )?;

    let (else_t, else_body) = opt_else
        .map(|(else_t, else_body)| (Some(else_t), else_body))
        .unwrap_or_else(|| (None, None));

    Ok(Builder::case(
        case_t,
        None,
        when_bodies,
        else_t,
        else_body,
        end_t,
    ))
}

fn parse_case_in(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (case_t, expr, _opt_terms, p_case_body, end_t) = all_of!(
        "k_case expr_value opt_terms p_case_body k_end",
        parse_k_case(parser),
        parser.parse_expr_value(),
        parser.parse_opt_terms(),
        parser.parse_p_case_body(),
        parser.parse_k_end(),
    )?;

    todo!(
        "{:?} {:?} {:?} {:?} {:?}",
        case_t,
        expr,
        _opt_terms,
        p_case_body,
        end_t
    )
}

fn parse_k_case(parser: &mut Parser) -> ParseResult<Token> {
    parser.try_token(TokenKind::kCASE)
}

fn parse_when_bodies(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    at_least_once!("when bodies", parse_when_body(parser))
}

fn parse_when_body(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (when_t, patterns, then_t, body) = all_of!(
        "case when body",
        parse_k_when(parser),
        parse_case_args(parser),
        parser.parse_then(),
        parser.try_compstmt(),
    )?;

    Ok(Builder::when(when_t, patterns, then_t, body))
}

fn parse_k_when(parser: &mut Parser) -> ParseResult<Token> {
    parser.try_token(TokenKind::kWHEN)
}

fn parse_case_args(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    at_least_once!("case args", parse_case_arg(parser))
}
fn parse_case_arg(parser: &mut Parser) -> ParseResult<Box<Node>> {
    one_of!(
        "case arg",
        checkpoint = parser.new_checkpoint(),
        {
            let (star_t, value) = all_of!(
                "*arg",
                parser.try_token(TokenKind::tSTAR),
                parser.parse_arg_value(),
            )?;

            Ok(Builder::splat(star_t, value))
        },
        parser.parse_arg_value(),
    )
}
