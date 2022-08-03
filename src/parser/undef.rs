use crate::{
    builder::Builder,
    parser::{ParseError, ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_undef(&mut self) -> ParseResult<Box<Node>> {
        let (undef_t, names) = self
            .all_of("undef ...")
            .and(|| self.try_token(TokenKind::kUNDEF))
            .and(|| self.parse_names())
            .stop()?;

        Ok(Builder::undef(undef_t, names))
    }

    fn parse_names(&mut self) -> ParseResult<Vec<Node>> {
        let mut names = vec![];
        let mut commas = vec![];

        let fitem = self.parse_fitem()?;
        names.push(*fitem);

        loop {
            if self.current_token().is(TokenKind::tCOMMA) {
                commas.push(self.current_token());
                // consume
                self.skip_token();
            } else {
                break;
            }
            match self.parse_fitem() {
                Ok(fitem) => names.push(*fitem),
                Err(error) => {
                    // got comma, but no `fitem`
                    return Err(ParseError::seq_error::<Vec<Node>, _>(
                        "list of undef items",
                        (names, commas),
                        error,
                    ));
                }
            }
        }

        Ok(names)
    }
}

#[test]
fn test_undef() {
    use crate::testing::assert_parses;
    assert_parses!(
        parse_undef,
        b"undef a, :b, c",
        r#"
s(:undef,
  s(:sym, "a"),
  s(:sym, "b"),
  s(:sym, "c"))
        "#
    );
}
