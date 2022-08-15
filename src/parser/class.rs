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
    pub(crate) fn parse_class(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "class definition",
            checkpoint = self.new_checkpoint(),
            {
                let (class_t, cpath, superclass, body, end_t) = all_of!(
                    "normal class definition",
                    self.parse_k_class(),
                    self.parse_cpath(),
                    self.try_superclass(),
                    self.try_bodystmt(),
                    self.parse_k_end(),
                )?;

                todo!(
                    "{:?} {:?} {:?} {:?} {:?}",
                    class_t,
                    cpath,
                    superclass,
                    body,
                    end_t
                )
            },
            {
                let (klass_t, lshift_t, expr, _term, body, end_t) = all_of!(
                    "singleton class",
                    self.parse_k_class(),
                    self.try_token(TokenKind::tLSHFT),
                    self.parse_expr(),
                    self.try_term(),
                    self.try_bodystmt(),
                    self.parse_k_end(),
                )?;

                todo!(
                    "{:?} {:?} {:?} {:?} {:?} {:?}",
                    klass_t,
                    lshift_t,
                    expr,
                    _term,
                    body,
                    end_t
                )
            },
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

    fn try_superclass(&mut self) -> ParseResult<Option<Box<Node>>> {
        todo!("parser.try_superclass")
    }

    fn parse_k_class(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kCLASS)
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_cpath_global_const() {
        assert_parses!(
            parse_cpath,
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
            parse_cpath,
            b"Foo::Bar",
            r#"
s(:const,
  s(:const, nil, "Foo"), "Bar")
"#
        )
    }

    #[test]
    fn test_cpath_simple() {
        assert_parses!(parse_cpath, b"Foo", r#"s(:const, nil, "Foo")"#)
    }
}
