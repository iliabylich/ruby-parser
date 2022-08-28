use crate::{
    builder::Builder,
    parser::{macros::one_of, ParseResult, Parser},
    token::TokenKind,
    Node,
};

use super::macros::{all_of, at_least_once, maybe::maybe};

impl Parser {
    pub(crate) fn parse_strings(&mut self) -> ParseResult<Box<Node>> {
        one_of!("strings", parse_char(self), parse_string_seq(self),)
    }

    // This rule can be `none`
    pub(crate) fn parse_string_contents(&mut self) -> ParseResult<Vec<Node>> {
        let contents = maybe!(at_least_once!(
            "string_contents",
            self.parse_string_content()
        ))?;

        Ok(contents.unwrap_or_else(|| vec![]))
    }

    pub(crate) fn parse_string_content(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "string_content",
            checkpoint = self.new_checkpoint(),
            {
                let string_content_t = self.try_token(TokenKind::tSTRING_CONTENT)?;
                Ok(Builder::string_internal(string_content_t, self.buffer()))
            },
            {
                let (_string_dvar_t, string_dvar) = all_of!(
                    "tSTRING_DVAR string_dvar",
                    self.try_token(TokenKind::tSTRING_DVAR),
                    parse_string_dvar(self),
                )?;

                Ok(string_dvar)
            },
            {
                let (begin_t, compstmt, end_t) = all_of!(
                    "tSTRING_DBEG compstmt tSTRING_DEND",
                    self.try_token(TokenKind::tSTRING_DBEG),
                    self.try_compstmt(),
                    self.expect_token(TokenKind::tSTRING_DEND),
                )?;

                let stmts = if let Some(compstmt) = compstmt {
                    vec![*compstmt]
                } else {
                    vec![]
                };

                Ok(Builder::begin(begin_t, stmts, end_t))
            },
        )
    }
}

fn parse_char(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let char_t = parser.try_token(TokenKind::tCHAR)?;
    Ok(Builder::character(char_t))
}

fn parse_string_seq(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let parts = at_least_once!("string", parse_string1(parser))?;

    Ok(Builder::string_compose(None, parts, None))
}

fn parse_string1(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (begin_t, parts, end_t) = all_of!(
        "string1",
        one_of!(
            "string1 begin",
            parser.try_token(TokenKind::tSTRING_BEG),
            parser.try_token(TokenKind::tDSTRING_BEG),
            parser.try_token(TokenKind::tHEREDOC_BEG),
        ),
        parser.parse_string_contents(),
        parser.expect_token(TokenKind::tSTRING_END),
    )?;

    // TODO: dedent_heredoc
    Ok(Builder::string_compose(Some(begin_t), parts, Some(end_t)))
}

fn parse_string_dvar(parser: &mut Parser) -> ParseResult<Box<Node>> {
    one_of!(
        "string_dvar",
        parser.parse_gvar(),
        parser.parse_ivar(),
        parser.parse_cvar(),
        parser.parse_back_ref(),
    )
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_char() {
        assert_parses!(parse_strings, b"?\\u0001", "s(:str, \"\\u{1}\")")
    }

    #[test]
    fn test_string1_plain() {
        assert_parses!(parse_strings, b"'foo'", "s(:str, \"foo\")");
    }

    #[test]
    fn test_string1_interp() {
        assert_parses!(
            parse_strings,
            b"\"foo #{42} #@bar\"",
            r#"
s(:dstr,
  s(:str, "foo "),
  s(:begin,
    s(:int, "42")),
  s(:str, " "),
  s(:ivar, "@bar"))
            "#
        );
    }
}
