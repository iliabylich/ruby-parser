use crate::{
    builder::Builder,
    parser::{macros::all_of, ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

use super::macros::one_of;

impl Parser {
    pub(crate) fn parse_if_expr(&mut self) -> ParseResult<Box<Node>> {
        let (if_t, cond, then_t, if_true, (else_t, if_false), end_t) = all_of!(
            "if expr",
            parse_k_if(self),
            self.parse_expr_value(),
            self.parse_then(),
            self.try_compstmt(),
            parse_if_tail(self),
            self.parse_k_end(),
        )?;

        Ok(Builder::condition(
            if_t,
            cond,
            then_t,
            if_true,
            else_t,
            if_false,
            Some(end_t),
        ))
    }

    pub(crate) fn parse_unless_expr(&mut self) -> ParseResult<Box<Node>> {
        let (cond_t, cond, then_t, if_false, opt_else, end_t) = all_of!(
            "if expr",
            parse_k_unless(self),
            self.parse_expr_value(),
            self.parse_then(),
            self.try_compstmt(),
            self.try_opt_else(),
            self.parse_k_end(),
        )?;

        let (else_t, if_true) = match opt_else {
            Some((else_t, body)) => (Some(else_t), body),
            None => (None, None),
        };

        Ok(Builder::condition(
            cond_t,
            cond,
            then_t,
            if_true,
            else_t,
            if_false,
            Some(end_t),
        ))
    }
}

fn parse_if_tail(parser: &mut Parser) -> ParseResult<(Option<Token>, Option<Box<Node>>)> {
    one_of!(
        "if tail",
        checkpoint = parser.new_checkpoint(),
        parse_if_tail_last(parser),
        parse_if_tail_recursive(parser),
    )
}

fn parse_if_tail_last(parser: &mut Parser) -> ParseResult<(Option<Token>, Option<Box<Node>>)> {
    let opt_else = parser.try_opt_else()?;
    match opt_else {
        Some((else_t, else_body)) => Ok((Some(else_t), else_body)),
        None => Ok((None, None)),
    }
}

fn parse_if_tail_recursive(parser: &mut Parser) -> ParseResult<(Option<Token>, Option<Box<Node>>)> {
    let (elsif_t, cond_t, then_t, else_body, (trailing_else_t, trailing_else_body)) = all_of!(
        "k_elsif expr_value then compstmt if_tail",
        parse_k_elsif(parser),
        parser.parse_expr_value(),
        parser.parse_then(),
        parser.try_compstmt(),
        parse_if_tail(parser),
    )?;

    Ok((
        Some(elsif_t),
        Some(Builder::condition(
            elsif_t,
            cond_t,
            then_t,
            else_body,
            trailing_else_t,
            trailing_else_body,
            None,
        )),
    ))
}

fn parse_k_if(parser: &mut Parser) -> ParseResult<Token> {
    parser.try_token(TokenKind::kIF)
}

fn parse_k_unless(parser: &mut Parser) -> ParseResult<Token> {
    parser.try_token(TokenKind::kUNLESS)
}

fn parse_k_elsif(parser: &mut Parser) -> ParseResult<Token> {
    parser.try_token(TokenKind::kELSIF)
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_if() {
        assert_parses!(
            parse_if_expr,
            b"if 1; 2; else; 3; end",
            r#"
s(:if,
  s(:int, "1"),
  s(:int, "2"),
  s(:int, "3"))
            "#
        )
    }

    #[test]
    fn test_unless() {
        assert_parses!(
            parse_unless_expr,
            b"unless 1; 2; else; 3; end",
            r#"
s(:if,
  s(:int, "1"),
  s(:int, "3"),
  s(:int, "2"))
            "#
        )
    }
}
