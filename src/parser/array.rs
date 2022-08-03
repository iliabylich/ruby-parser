use crate::{
    builder::Builder,
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_array(&mut self) -> ParseResult<Box<Node>> {
        let (lbrack_t, elements, rbrack_t) = self
            .all_of("array")
            .and(|| self.try_token(TokenKind::tLBRACK))
            .and(|| parse_aref_args(self))
            .and(|| self.expect_token(TokenKind::tRBRACK))
            .stop()?;

        Ok(Builder::array(Some(lbrack_t), elements, Some(rbrack_t)))
    }
}

fn parse_aref_args(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    let mut head = parser.parse_args()?;
    let mut tail = parser.parse_assocs()?;
    let _trailer = parser.try_trailer();

    head.append(&mut tail);
    Ok(head)
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_array_simple() {
        assert_parses!(parse_array, b"[1, 2, 3]", "TODO")
    }

    #[test]
    fn test_array_mixed() {
        assert_parses!(parse_array, b"[1, 2, 3, 4 => 5]", "TODO")
    }
}
