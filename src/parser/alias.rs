use crate::{
    builder::{Builder, Constructor},
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl<C: Constructor> Parser<C> {
    pub(crate) fn try_alias(&mut self) -> ParseResult<Box<Node>> {
        let (alias_t, (lhs, rhs)) = self
            .all_of("alias statement")
            .and(|| self.try_token(TokenKind::kALIAS))
            .and(|| try_alias_args(self))
            .stop()?;
        Ok(Builder::<C>::alias(alias_t, lhs, rhs))
    }
}

fn try_alias_args<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<(Box<Node>, Box<Node>)> {
    parser
        .one_of("alias arguments")
        .or_else(|| try_fitem_fitem(parser))
        .or_else(|| try_gvar_gvar(parser))
        .required()
        .compact()
        .stop()
}

fn try_fitem_fitem<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<(Box<Node>, Box<Node>)> {
    parser
        .all_of("fitem -> fitem")
        .and(|| parser.try_fitem())
        .and(|| parser.try_fitem())
        .stop()
}

fn try_gvar_gvar<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<(Box<Node>, Box<Node>)> {
    parser
        .all_of("gvar -> [gvar | back ref | nth ref]")
        .and(|| parser.try_gvar())
        .and(|| {
            parser
                .one_of("gvar rhs")
                .or_else(|| parser.try_gvar())
                .or_else(|| parser.try_back_ref())
                .or_else(|| parser.try_nth_ref())
                .required()
                .stop()
        })
        .stop()
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_parses, assert_parses_with_error};

    #[test]
    fn test_alias_name_to_name() {
        assert_parses!(
            try_alias,
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
            try_alias,
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
            try_alias,
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
            try_alias,
            b"",
            "
SEQUENCE (0) alias statement (got [])
    TOKEN (0) expected kALIAS, got tEOF (at 0)
"
        );
    }

    #[test]
    fn test_only_alias() {
        assert_parses_with_error!(
            try_alias,
            b"alias $foo",
            "
SEQUENCE (1) alias statement (got [Token(Token { kind: kALIAS, loc: 0...5, value: None })])
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
