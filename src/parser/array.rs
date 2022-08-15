use crate::{
    builder::Builder,
    parser::{macros::all_of, ParseError, ParseResult, Parser},
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
    let mut nodes = vec![];
    let mut commas = vec![];

    let item = parse_aref_args_item(parser)?;
    nodes.push(*item);

    loop {
        if parser.current_token().is(TokenKind::tCOMMA) {
            commas.push(parser.current_token());
            parser.skip_token();
        } else {
            break;
        }

        match parse_aref_args_item(parser) {
            Ok(item) => nodes.push(*item),
            Err(error) => return Err(ParseError::seq_error("aref args", (nodes, commas), error)),
        }
    }
    let _trailer = parser.try_trailer();

    Ok(nodes)
}

fn parse_aref_args_item(parser: &mut Parser) -> ParseResult<Box<Node>> {
    parser
        .one_of("areg_args item")
        .or_else(|| parser.parse_assoc())
        .or_else(|| parser.parse_arg())
        .stop()
}

fn try_args(parser: &mut Parser) -> ParseResult<Option<Vec<Node>>> {
    parser
        .one_of("[args]")
        .or_else(|| parser.parse_args().map(|v| Some(v)))
        .or_else(|| Ok(None))
        .stop()
}

fn try_assocs(parser: &mut Parser) -> ParseResult<Option<Vec<Node>>> {
    parser
        .one_of("[assocs]")
        .or_else(|| parser.parse_assocs().map(|v| Some(v)))
        .or_else(|| Ok(None))
        .stop()
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_array_simple() {
        assert_parses!(
            parse_array,
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
            parse_array,
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
