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
        let begin_t = self.try_token(TokenKind::klBEGIN)?;
        let lcurly_t = self.expect_token(TokenKind::tLCURLY);
        let top_compstmt = self.try_top_compstmt()?;
        let rcurly_t = self.expect_token(TokenKind::tRCURLY);
        Ok(Builder::<C>::preexe(
            begin_t,
            lcurly_t,
            top_compstmt,
            rcurly_t,
        ))
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
