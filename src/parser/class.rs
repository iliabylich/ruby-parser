use crate::{
    builder::Builder,
    parser::{
        macros::{all_of, maybe, one_of},
        ParseResult, Parser,
    },
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_class(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "class definition",
            checkpoint = self.new_checkpoint(),
            parse_class(self),
            parse_singleton_class(self),
        )
    }

    pub(crate) fn parse_cpath(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "cname",
            checkpoint = self.new_checkpoint(),
            {
                let (colon2_t, name_t) = self.parse_colon2_const()?;
                Ok(Builder::const_global(colon2_t, name_t, self.buffer()))
            },
            self.parse_primary_value(),
            {
                let name_t = self.parse_cname()?;
                Ok(Builder::const_(name_t, self.buffer()))
            },
        )
    }

    fn try_superclass(&mut self) -> ParseResult<Option<(Token, Box<Node>)>> {
        let superclass = maybe!(all_of!(
            "superclass",
            self.try_token(TokenKind::tLT),
            self.parse_expr_value(),
            self.parse_term(),
        ))?;

        match superclass {
            Some((lt_t, superclass, _term)) => Ok(Some((lt_t, superclass))),
            None => Ok(None),
        }
    }
}

fn parse_k_class(parser: &mut Parser) -> ParseResult<Token> {
    parser.try_token(TokenKind::kCLASS)
}

fn parse_class(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (class_t, name, superclass, body, end_t) = all_of!(
        "normal class definition",
        parse_k_class(parser),
        parser.parse_cpath(),
        parser.try_superclass(),
        parser.try_bodystmt(),
        parser.parse_k_end(),
    )?;

    let (lt_t, superclass) = match superclass {
        Some((lt_t, superclass)) => (Some(lt_t), Some(superclass)),
        None => (None, None),
    };

    Ok(Builder::def_class(
        class_t, name, lt_t, superclass, body, end_t,
    ))
}

fn parse_singleton_class(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (class_t, lshift_t, expr, _term, body, end_t) = all_of!(
        "singleton class",
        parse_k_class(parser),
        parser.try_token(TokenKind::tLSHFT),
        parser.parse_expr(),
        parser.parse_term(),
        parser.try_bodystmt(),
        parser.parse_k_end(),
    )?;

    Ok(Builder::def_sclass(class_t, lshift_t, expr, body, end_t))
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_cpath_global_const() {
        assert_parses!(
            Parser::parse_cpath,
            b"::Foo",
            r#"
s(:const,
  s(:cbase), "Foo")
"#
        )
    }

    #[test]
    fn test_cpath_primary() {
        assert_parses!(
            Parser::parse_cpath,
            b"Foo::Bar",
            r#"
s(:const,
  s(:const, nil, "Foo"), "Bar")
"#
        )
    }

    #[test]
    fn test_cpath_simple() {
        assert_parses!(Parser::parse_cpath, b"Foo", r#"s(:const, nil, "Foo")"#)
    }

    #[test]
    fn test_class() {
        assert_parses!(
            Parser::parse_class,
            b"class Foo; 42; end",
            r#"
s(:class,
  s(:const, nil, "Foo"), nil,
  s(:int, "42"))
            "#
        )
    }

    #[test]
    fn test_sclass() {
        assert_parses!(
            Parser::parse_class,
            b"class << Foo; 42; end",
            r#"
s(:sclass,
  s(:const, nil, "Foo"),
  s(:int, "42"))
            "#
        )
    }
}
