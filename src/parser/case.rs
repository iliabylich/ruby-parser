use crate::{
    builder::Builder,
    parser::{ParseError, ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_case(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("case expr")
            .or_else(|| {
                let (case_t, expr, _terms, when_bodies, opt_else, end_t) = self
                    .all_of("k_case expr_value opt_terms case_body k_end")
                    .and(|| parse_k_case(self))
                    .and(|| self.parse_expr_value())
                    .and(|| self.parse_opt_terms())
                    .and(|| parse_when_bodies(self))
                    .and(|| self.try_opt_else())
                    .and(|| self.parse_k_end())
                    .stop()?;

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
            })
            .or_else(|| {
                let (case_t, _opt_terms, when_bodies, opt_else, end_t) = self
                    .all_of("k_case opt_terms case_body k_end")
                    .and(|| parse_k_case(self))
                    .and(|| self.parse_opt_terms())
                    .and(|| parse_when_bodies(self))
                    .and(|| self.try_opt_else())
                    .and(|| self.parse_k_end())
                    .stop()?;

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
            })
            .or_else(|| {
                let (case_t, expr, _opt_terms, p_case_body, end_t) = self
                    .all_of("k_case expr_value opt_terms p_case_body k_end")
                    .and(|| parse_k_case(self))
                    .and(|| self.parse_expr_value())
                    .and(|| self.parse_opt_terms())
                    .and(|| self.parse_p_case_body())
                    .and(|| self.parse_k_end())
                    .stop()?;

                todo!(
                    "{:?} {:?} {:?} {:?} {:?}",
                    case_t,
                    expr,
                    _opt_terms,
                    p_case_body,
                    end_t
                )
            })
            .stop()
    }
}

fn parse_k_case(parser: &mut Parser) -> ParseResult<Token> {
    parser.try_token(TokenKind::kCASE)
}

fn parse_when_bodies(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    let mut nodes = vec![];
    loop {
        match parse_when_body(parser) {
            Ok(when_body) => nodes.push(*when_body),
            Err(err) => match err.strip_lookaheads() {
                Some(error) => return Err(ParseError::seq_error("when bodies", nodes, error)),
                None => break,
            },
        }
    }
    Ok(nodes)
}

fn parse_when_body(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (when_t, patterns, then_t, body) = parser
        .all_of("case when body")
        .and(|| parse_k_when(parser))
        .and(|| parse_case_args(parser))
        .and(|| parser.parse_then())
        .and(|| parser.try_compstmt())
        .stop()?;

    Ok(Builder::when(when_t, patterns, then_t, body))
}

fn parse_k_when(parser: &mut Parser) -> ParseResult<Token> {
    parser.try_token(TokenKind::kWHEN)
}

fn parse_case_args(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    let mut nodes = vec![];
    let mut commas = vec![];

    let node = parse_case_arg(parser)?;
    nodes.push(*node);

    loop {
        if parser.current_token().is(TokenKind::tCOMMA) {
            commas.push(parser.current_token());
            parser.skip_token();
        } else {
            break;
        }
        match parse_case_arg(parser) {
            Ok(node) => nodes.push(*node),
            Err(error) => return Err(ParseError::seq_error("case args", (nodes, commas), error)),
        }
    }
    Ok(nodes)
}
fn parse_case_arg(parser: &mut Parser) -> ParseResult<Box<Node>> {
    parser
        .one_of("case arg")
        .or_else(|| {
            let (star_t, value) = parser
                .all_of("*arg")
                .and(|| parser.try_token(TokenKind::tSTAR))
                .and(|| parser.parse_arg_value())
                .stop()?;

            Ok(Builder::splat(star_t, value))
        })
        .or_else(|| parser.parse_arg_value())
        .stop()
}
