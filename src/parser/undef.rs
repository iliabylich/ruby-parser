use crate::{
    builder::{Builder, Constructor},
    parser::{ParseError, ParseResult, Parser},
    token::TokenKind,
    transactions::StepData,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_undef(&mut self) -> ParseResult<Box<Node>> {
        let (undef_t, names) = self
            .all_of("undef ...")
            .and(|| self.try_token(TokenKind::kUNDEF))
            .and(|| self.try_names())
            .stop()?;

        Ok(Builder::<C>::undef(undef_t, names))
    }

    fn try_names(&mut self) -> ParseResult<Vec<Node>> {
        let mut names = vec![];
        let mut commas = vec![];

        let fitem = self.try_fitem()?;
        names.push(*fitem);

        loop {
            if self.current_token().is(TokenKind::tCOMMA) {
                commas.push(self.current_token());
                // consume
                self.skip_token();
            } else {
                break;
            }
            match self.try_fitem() {
                Ok(fitem) => names.push(*fitem),
                Err(error) => {
                    // got comma, but no `fitem`
                    let mut steps = vec![];
                    steps.extend(names.into_iter().map(|name| StepData::from(Box::new(name))));
                    steps.extend(commas.into_iter().map(|comma| StepData::from(comma)));
                    return Err(ParseError::SeqError {
                        name: "list of undef items",
                        steps,
                        error: Box::new(error),
                    });
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
        try_undef,
        b"undef a, :b, c",
        r#"
s(:undef,
  s(:sym, "a"),
  s(:sym, "b"),
  s(:sym, "c"))
        "#
    );
}
