use crate::{
    builder::Builder,
    parser::{
        macros::{all_of, one_of},
        ParseResult, Parser,
    },
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_method(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "method definition",
            checkpoint = self.new_checkpoint(),
            parse_instance_method_definition(self),
            parse_singleton_method_definition(self),
        )
    }

    pub(crate) fn parse_defn_head(&mut self) -> ParseResult<(Token, Token)> {
        all_of!(
            "instance method definition start",
            parse_k_def(self),
            self.parse_def_name(),
        )
    }

    pub(crate) fn parse_defs_head(&mut self) -> ParseResult<(Token, Box<Node>, Token, Token)> {
        all_of!(
            "singleton method definition start",
            parse_k_def(self),
            parse_singleton(self),
            self.parse_dot_or_colon(),
            self.parse_def_name(),
        )
    }
}

fn parse_instance_method_definition(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let ((def_t, name_t), args, body, end_t) = all_of!(
        "instance method definition",
        parser.parse_defn_head(),
        try_f_arglist(parser),
        parser.try_bodystmt(),
        parser.parse_k_end(),
    )?;

    Ok(Builder::def_method(
        def_t,
        name_t,
        args,
        body,
        end_t,
        parser.buffer(),
    ))
}

fn parse_singleton_method_definition(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let ((def_t, definee, dot_t, name_t), args, body, end_t) = all_of!(
        "singleton method definition",
        parser.parse_defs_head(),
        try_f_arglist(parser),
        parser.try_bodystmt(),
        parser.parse_k_end(),
    )?;

    Ok(Builder::def_singleton(
        def_t,
        definee,
        dot_t,
        name_t,
        args,
        body,
        end_t,
        parser.buffer(),
    ))
}

fn parse_k_def(parser: &mut Parser) -> ParseResult<Token> {
    parser.try_token(TokenKind::kDEF)
}

fn try_f_arglist(parser: &mut Parser) -> ParseResult<Option<Box<Node>>> {
    one_of!(
        "f_arglist",
        checkpoint = parser.new_checkpoint(),
        parser.parse_f_paren_args(),
        {
            let (f_args, _term) =
                all_of!("f_args term", parser.parse_f_args(), parser.parse_term(),)?;
            Ok(Builder::args(None, f_args, None))
        },
    )
}

fn parse_singleton(parser: &mut Parser) -> ParseResult<Box<Node>> {
    one_of!(
        "singleton",
        checkpoint = parser.new_checkpoint(),
        parser.parse_var_ref(),
        {
            let (_lparen_t, expr, _rparen_t) = all_of!(
                "(expr)",
                parser.try_token(TokenKind::tLPAREN),
                parser.parse_expr(),
                parser.parse_rparen(),
            )?;
            // TODO: check value_expr

            Ok(expr)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{parse_instance_method_definition, parse_singleton_method_definition};
    use crate::testing::assert_parses;

    #[test]
    fn test_instance_method_def() {
        assert_parses!(
            parse_instance_method_definition,
            b"def foo; 42; end",
            r#"
s(:def, "foo", nil,
  s(:int, "42"))
            "#
        )
    }

    #[test]
    fn test_singleton_method_def() {
        assert_parses!(
            parse_singleton_method_definition,
            b"def self.foo; 42; end",
            r#"
s(:defs,
  s(:self), "foo", nil,
  s(:int, "42"))
            "#
        )
    }

    #[test]
    fn test_singleton_method_def_on_expr() {
        assert_parses!(
            parse_singleton_method_definition,
            b"def (foo).bar; 42; end",
            r#"
s(:defs,
  s(:lvar, "foo"), "bar", nil,
  s(:int, "42"))
            "#
        )
    }
}
