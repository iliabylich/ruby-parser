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
    pub(crate) fn parse_symbol(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "symbol",
            checkpoint = self.new_checkpoint(),
            self.parse_ssym(),
            self.parse_dsym(),
        )
    }

    fn parse_ssym(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "static symbol",
            checkpoint = self.new_checkpoint(),
            {
                let (colon_t, sym_t) = all_of!(":sym", self.try_token(TokenKind::tCOLON), {
                    one_of!(
                        "static symbol value",
                        checkpoint = self.new_checkpoint(),
                        self.parse_fname(),
                        self.try_token(TokenKind::tIVAR),
                        self.try_token(TokenKind::tCVAR),
                        self.try_token(TokenKind::tGVAR),
                    )
                },)?;

                Ok(Builder::symbol(colon_t, sym_t, self.buffer()))
            },
            {
                let (begin_t, parts, end_t) = all_of!(
                    "dynamic symbol value",
                    self.try_token(TokenKind::tSYMBEG),
                    self.parse_string_contents(),
                    self.expect_token(TokenKind::tSTRING_END),
                )?;

                Ok(Builder::symbol_compose(begin_t, parts, end_t))
            },
        )
    }

    fn parse_dsym(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, parts, end_t) = all_of!(
            "dynamic symbol",
            self.try_token(TokenKind::tDSYMBEG),
            self.parse_string_contents(),
            self.expect_token(TokenKind::tSTRING_END),
        )?;

        let node = Builder::symbol_compose(begin_t, parts, end_t);
        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use crate::{testing::assert_parses, testing::assert_parses_with_error};

    #[test]
    fn test_ssym() {
        assert_parses!(parse_ssym, b":foo", "s(:sym, \"foo\")")
    }

    #[test]
    fn test_ssym_ivar() {
        assert_parses!(parse_ssym, b":@ivar", "s(:sym, \"@ivar\")")
    }

    #[test]
    fn test_ssym_keyword() {
        assert_parses!(parse_ssym, b":super", "s(:sym, \"super\")")
    }

    #[test]
    fn test_ssym_only_colon() {
        let parser = assert_parses_with_error!(parse_ssym, b":");
        // `:` is consumed
        assert_eq!(parser.lexer.buffer().pos(), 1);
    }

    #[test]
    fn test_ssym_no_colon() {
        let parser = assert_parses_with_error!(
            parse_ssym,
            b"",
            "
ONEOF (0) static symbol
    SEQUENCE (0) :sym (got [])
        TOKEN (0) expected tCOLON, got tEOF (at 0)
    SEQUENCE (0) dynamic symbol value (got [])
        TOKEN (0) expected tSYMBEG, got tEOF (at 0)
"
        );
        assert_eq!(parser.lexer.buffer().pos(), 0);
    }

    #[test]
    fn test_ssym_quoted() {
        assert_parses!(parse_ssym, b":'foo'", "s(:sym, \"foo\")")
    }

    #[test]
    fn test_dsym() {
        assert_parses!(parse_dsym, b":\"foo\"", "TODO")
    }

    #[test]
    fn test_dsym_only_colon() {
        let parser = assert_parses_with_error!(
            parse_dsym,
            b":",
            "
SEQUENCE (0) dynamic symbol (got [])
    TOKEN (0) expected tDSYMBEG, got tCOLON (at 0)
"
        );
        // `:` is consumed
        assert_eq!(parser.lexer.buffer().pos(), 1);
    }

    #[test]
    fn test_dsym_no_colon() {
        assert_parses_with_error!(
            parse_dsym,
            b"",
            "
SEQUENCE (0) dynamic symbol (got [])
    TOKEN (0) expected tDSYMBEG, got tEOF (at 0)
"
        );
    }
}
