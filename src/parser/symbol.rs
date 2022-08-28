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
            parse_ssym(self),
            parse_dsym(self),
        )
    }
}

fn parse_ssym(parser: &mut Parser) -> ParseResult<Box<Node>> {
    one_of!(
        "static symbol",
        checkpoint = parser.new_checkpoint(),
        {
            let (colon_t, sym_t) = all_of!(":sym", parser.try_token(TokenKind::tCOLON), {
                one_of!(
                    "static symbol value",
                    checkpoint = parser.new_checkpoint(),
                    parser.parse_fname(),
                    parser.try_token(TokenKind::tIVAR),
                    parser.try_token(TokenKind::tCVAR),
                    parser.try_token(TokenKind::tGVAR),
                )
            },)?;

            Ok(Builder::symbol(colon_t, sym_t, parser.buffer()))
        },
        {
            let (begin_t, parts, end_t) = all_of!(
                "dynamic symbol value",
                parser.try_token(TokenKind::tSYMBEG),
                parser.parse_string_contents(),
                parser.expect_token(TokenKind::tSTRING_END),
            )?;

            Ok(Builder::symbol_compose(begin_t, parts, end_t))
        },
    )
}

fn parse_dsym(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (begin_t, parts, end_t) = all_of!(
        "dynamic symbol",
        parser.try_token(TokenKind::tDSYMBEG),
        parser.parse_string_contents(),
        parser.expect_token(TokenKind::tSTRING_END),
    )?;

    let node = Builder::symbol_compose(begin_t, parts, end_t);
    Ok(node)
}

#[cfg(test)]
mod tests {
    use crate::{testing::assert_parses, testing::assert_parses_with_error};

    #[test]
    fn test_ssym() {
        assert_parses!(parse_symbol, b":foo", "s(:sym, \"foo\")")
    }

    #[test]
    fn test_ssym_ivar() {
        assert_parses!(parse_symbol, b":@ivar", "s(:sym, \"@ivar\")")
    }

    #[test]
    fn test_ssym_keyword() {
        assert_parses!(parse_symbol, b":super", "s(:sym, \"super\")")
    }

    #[test]
    fn test_ssym_only_colon() {
        let parser = assert_parses_with_error!(parse_symbol, b":");
        // `:` is consumed
        assert_eq!(parser.lexer.buffer().pos(), 1);
    }

    #[test]
    fn test_ssym_no_colon() {
        let parser = assert_parses_with_error!(
            parse_symbol,
            b"",
            "
ONEOF (0) symbol
    ONEOF (0) static symbol
        SEQUENCE (0) :sym (got [])
            TOKEN (0) expected tCOLON, got tEOF (at 0)
        SEQUENCE (0) dynamic symbol value (got [])
            TOKEN (0) expected tSYMBEG, got tEOF (at 0)
    SEQUENCE (0) dynamic symbol (got [])
        TOKEN (0) expected tDSYMBEG, got tEOF (at 0)
"
        );
        assert_eq!(parser.lexer.buffer().pos(), 0);
    }

    #[test]
    fn test_ssym_quoted() {
        assert_parses!(parse_symbol, b":'foo'", "s(:sym, \"foo\")")
    }

    #[test]
    fn test_dsym() {
        assert_parses!(parse_symbol, b":\"foo\"", "s(:sym, \"foo\")")
    }

    #[test]
    fn test_dsym_interp() {
        assert_parses!(
            parse_symbol,
            b":\"foo#{42}bar\"",
            r#"
s(:dsym,
  s(:str, "foo"),
  s(:begin,
    s(:int, "42")),
  s(:str, "bar"))
            "#
        )
    }

    #[test]
    fn test_dsym_only_colon() {
        let parser = assert_parses_with_error!(
            parse_symbol,
            b":",
            "
ONEOF (1) symbol
    ONEOF (1) static symbol
        SEQUENCE (1) :sym (got [Token(Token { kind: tCOLON, loc: 0...1, value: None })])
            ONEOF (0) static symbol value
                ONEOF (0) fname
                    TOKEN (0) expected tIDENTIFIER, got tEOF (at 1)
                    TOKEN (0) expected tCONSTANT, got tEOF (at 1)
                    TOKEN (0) expected tFID, got tEOF (at 1)
                    ONEOF (0) operation

                    ONEOF (0) reserved word

                TOKEN (0) expected tIVAR, got tEOF (at 1)
                TOKEN (0) expected tCVAR, got tEOF (at 1)
                TOKEN (0) expected tGVAR, got tEOF (at 1)
"
        );
        // `:` is consumed
        assert_eq!(parser.lexer.buffer().pos(), 1);
    }

    #[test]
    fn test_dsym_no_colon() {
        assert_parses_with_error!(
            parse_symbol,
            b"",
            "
ONEOF (0) symbol
    ONEOF (0) static symbol
        SEQUENCE (0) :sym (got [])
            TOKEN (0) expected tCOLON, got tEOF (at 0)
        SEQUENCE (0) dynamic symbol value (got [])
            TOKEN (0) expected tSYMBEG, got tEOF (at 0)
    SEQUENCE (0) dynamic symbol (got [])
        TOKEN (0) expected tDSYMBEG, got tEOF (at 0)
"
        );
    }
}
