use crate::{
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn try_array(&mut self) -> ParseResult<Box<Node>> {
        let (lbrack_t, elements, rbrack_t) = self
            .all_of("array")
            .and(|| self.try_token(TokenKind::tLBRACK))
            .and(|| try_aref_args(self))
            .and(|| self.expect_token(TokenKind::tRBRACK))
            .stop()?;

        todo!("array {:?} {:?} {:?}", lbrack_t, elements, rbrack_t);
    }
}

fn try_aref_args(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    let mut head = parser.try_args()?;
    let mut tail = parser.try_assocs()?;
    let _trailer = parser.try_opt_trailer();

    head.append(&mut tail);
    Ok(head)
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_array() {
        assert_parses!(try_array, b"[1, 2, 3, 4 => 5]", "TODO")
    }
}
