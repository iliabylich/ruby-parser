use crate::{
    builder::Builder,
    parser::{
        macros::{all_of, one_of},
        ParseResult, Parser,
    },
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_alias(&mut self) -> ParseResult<Box<Node>> {
        let (alias_t, (lhs, rhs)) = all_of!(
            "alias",
            self.try_token(TokenKind::kALIAS),
            parse_alias_args(self),
        )?;
        Ok(Builder::alias(alias_t, lhs, rhs))
    }
}

fn parse_alias_args(parser: &mut Parser) -> ParseResult<(Box<Node>, Box<Node>)> {
    one_of!(
        "alias arguments",
        parse_fitem_fitem(parser),
        parse_gvar_gvar(parser),
    )
}

fn parse_fitem_fitem(parser: &mut Parser) -> ParseResult<(Box<Node>, Box<Node>)> {
    all_of!("fitem -> fitem", parser.parse_fitem(), parser.parse_fitem(),)
}

fn parse_gvar_gvar(parser: &mut Parser) -> ParseResult<(Box<Node>, Box<Node>)> {
    all_of!(
        "gvar -> [gvar | back ref | nth ref]",
        parser.parse_gvar(),
        one_of!(
            "gvar rhs",
            parser.parse_gvar(),
            parser.parse_back_ref(),
            parser.parse_nth_ref(),
        ),
    )
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_parses, assert_parses_with_error};

    #[test]
    fn test_alias_name_to_name() {
        assert_parses!(
            parse_alias,
            b"alias foo bar",
            r#"
s(:alias,
  s(:sym, "foo"),
  s(:sym, "bar"))
            "#
        )
    }

    #[test]
    fn test_alias_sym_to_sym() {
        assert_parses!(
            parse_alias,
            b"alias :foo :bar",
            r#"
s(:alias,
  s(:sym, "foo"),
  s(:sym, "bar"))
            "#
        )
    }

    #[test]
    fn test_alias_gvar_to_gvar() {
        assert_parses!(
            parse_alias,
            b"alias $foo $bar",
            r#"
s(:alias,
  s(:gvar, "$foo"),
  s(:gvar, "$bar"))
            "#
        )
    }

    #[test]
    fn test_nothing() {
        assert_parses_with_error!(
            parse_alias,
            b"",
            "
SEQUENCE (0) alias (got [])
    TOKEN (0) expected kALIAS, got tEOF (at 0)
"
        );
    }

    #[test]
    fn test_only_alias() {
        assert_parses_with_error!(
            parse_alias,
            b"alias $foo",
            "
SEQUENCE (1) alias (got [Token(Token { kind: kALIAS, loc: 0...5, value: None })])
    ONEOF (1) alias arguments
        SEQUENCE (1) gvar -> [gvar | back ref | nth ref] (got [Node(Gvar(Gvar { name: StringContent { bytes: [36, 102, 111, 111] }, expression_l: 6...10 }))])
            ONEOF (0) gvar rhs
                TOKEN (0) expected tGVAR, got tEOF (at 10)
                TOKEN (0) expected tBACK_REF, got tEOF (at 10)
                TOKEN (0) expected tNTH_REF, got tEOF (at 10)
"
        );
    }
}
