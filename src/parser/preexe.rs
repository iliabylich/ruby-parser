use crate::{
    builder::{Builder, Constructor},
    parser::{ParseError, Parser},
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_preexe(&mut self) -> Result<Box<Node>, ParseError> {
        let (preexe_t, lcurly_t, body, rcurly_t) = self
            .all_of("preexe")
            .and(|| self.try_token(TokenKind::klBEGIN))
            .and(|| self.expect_token(TokenKind::tLCURLY))
            .and(|| self.try_top_compstmt())
            .and(|| self.expect_token(TokenKind::tRCURLY))
            .unwrap()?;

        Ok(Builder::<C>::preexe(preexe_t, lcurly_t, body, rcurly_t))
    }
}

#[test]
fn test_preexe() {
    use crate::parser::RustParser;
    let mut parser = RustParser::new(b"BEGIN {}").debug();
    assert_eq!(parser.try_preexe(), Err(ParseError::empty()));
    todo!("implement me");
}

#[test]
fn test_nothing() {
    use crate::{loc::loc, parser::RustParser};
    let mut parser = RustParser::new(b"");
    assert_eq!(
        parser.try_preexe(),
        Err(ParseError::TokenError {
            lookahead: true,
            expected: TokenKind::klBEGIN,
            got: TokenKind::tEOF,
            loc: loc!(0, 0),
        })
    );
}
