use crate::{
    builder::Builder,
    parser::{
        macros::{all_of, one_of, separated_by},
        ParseResult, Parser,
    },
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_array(&mut self) -> ParseResult<Box<Node>> {
        let (lbrack_t, elements, rbrack_t) = all_of!(
            "array",
            self.try_token(TokenKind::tLBRACK),
            parse_aref_args(self),
            self.expect_token(TokenKind::tRBRACK),
        )?;

        Ok(Builder::array(Some(lbrack_t), elements, Some(rbrack_t)))
    }
}

fn parse_aref_args(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    let (aref_args, _commas) = separated_by!(
        "aref args",
        checkpoint = parser.new_checkpoint(),
        item = parse_aref_args_item(parser),
        sep = parser.try_token(TokenKind::tCOMMA)
    )?;

    let _trailer = parser.try_trailer();

    Ok(aref_args)
}

fn parse_aref_args_item(parser: &mut Parser) -> ParseResult<Box<Node>> {
    one_of!(
        "aref_args item",
        checkpoint = parser.new_checkpoint(),
        parser.parse_assoc(),
        parser.parse_arg(),
    )
}

fn try_args(parser: &mut Parser) -> ParseResult<Option<Vec<Node>>> {
    one_of!(
        "[args]",
        checkpoint = parser.new_checkpoint(),
        parser.parse_args().map(|v| Some(v)),
        Ok(None),
    )
}

fn try_assocs(parser: &mut Parser) -> ParseResult<Option<Vec<Node>>> {
    one_of!(
        "[assocs]",
        checkpoint = parser.new_checkpoint(),
        parser.parse_assocs().map(|v| Some(v)),
        Ok(None),
    )
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_array_simple() {
        assert_parses!(
            Parser::parse_array,
            b"[1, 2, 3]",
            r#"
s(:array,
  s(:int, "1"),
  s(:int, "2"),
  s(:int, "3"))
            "#
        )
    }

    #[test]
    fn test_array_mixed() {
        assert_parses!(
            Parser::parse_array,
            b"[1, 2, 3, 4 => 5]",
            r#"
s(:array,
  s(:int, "1"),
  s(:int, "2"),
  s(:int, "3"),
  s(:pair,
    s(:int, "4"),
    s(:int, "5")))
            "#
        )
    }
}
