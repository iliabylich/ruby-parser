use crate::{
    builder::{Builder, Constructor},
    parser::{ParseResult, ParseResultApi, Parser},
    token::{Token, TokenKind},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_strings(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("strings")
            .or_else(|| self.try_char())
            .or_else(|| self.try_string_seq())
            .stop()
    }

    fn try_char(&mut self) -> ParseResult<Box<Node>> {
        let char_t = self.try_token(TokenKind::tCHAR)?;
        Ok(Builder::<C>::character(char_t))
    }

    fn try_string_seq(&mut self) -> ParseResult<Box<Node>> {
        let mut parts = vec![];

        let string = self.try_string1()?;
        parts.push(*string);

        loop {
            match self.try_string1().ignore_lookaheads()? {
                Some(string) => {
                    parts.push(*string);
                }
                None => {
                    // no match
                    break;
                }
            }
        }

        Ok(Builder::<C>::string_compose(None, parts, None))
    }

    fn try_string1(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, parts, end_t) = self
            .all_of("string1")
            .and(|| {
                self.one_of("string begin")
                    .or_else(|| self.try_token(TokenKind::tDSTRING_BEG))
                    .or_else(|| self.try_token(TokenKind::tSTRING_BEG))
                    .or_else(|| self.try_token(TokenKind::tHEREDOC_BEG))
                    .stop()
            })
            .and(|| self.try_string_contents())
            .and(|| self.expect_token(TokenKind::tSTRING_END))
            .stop()?;

        // TODO: dedent_heredoc
        Ok(Builder::<C>::string_compose(
            Some(begin_t),
            parts,
            Some(end_t),
        ))
    }

    // This rule can be `none`
    pub(crate) fn try_string_contents(&mut self) -> ParseResult<Vec<Node>> {
        let mut strings = vec![];
        loop {
            if self.current_token().is(TokenKind::tSTRING_END) {
                break;
            }

            match self.try_string_content().ignore_lookaheads()? {
                Some(string) => {
                    strings.push(*string);
                }
                None => {
                    // no match
                    break;
                }
            }
        }
        Ok(strings)
    }

    pub(crate) fn try_string_content(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("string content")
            .or_else(|| {
                let string_content_t = self.try_token(TokenKind::tSTRING_CONTENT)?;
                Ok(Builder::<C>::string_internal(
                    string_content_t,
                    self.buffer(),
                ))
            })
            .or_else(|| {
                let (string_dvar_t, string_dvar) = self
                    .all_of("string dvar")
                    .and(|| self.try_token(TokenKind::tSTRING_DVAR))
                    .and(|| self.try_string_dvar())
                    .stop()?;

                panic!(
                    "tSTRING_DVAR string_dvar {:?} {:?}",
                    string_dvar_t, string_dvar
                )
            })
            .or_else(|| {
                let (string_dbeg_t, compstmt, string_dend_t) = self
                    .all_of("#{ stmt }")
                    .and(|| self.try_token(TokenKind::tSTRING_DBEG))
                    .and(|| self.try_compstmt())
                    .and(|| self.expect_token(TokenKind::tSTRING_DEND))
                    .stop()?;

                panic!(
                    "tSTRING_DBEG compstmt tSTRING_DEND {:?} {:?} {:?}",
                    string_dbeg_t, compstmt, string_dend_t
                )
            })
            .stop()
    }

    fn try_string_dvar(&mut self) -> ParseResult<Token> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        loc::loc, nodes::Str, parser::ParseError, string_content::StringContent, Node, RustParser,
    };

    #[test]
    fn test_char() {
        let mut parser = RustParser::new(b"?\\u0001");
        assert_eq!(
            parser.try_strings(),
            Ok(Box::new(Node::Str(Str {
                value: StringContent::from("\u{0001}"),
                begin_l: Some(loc!(0, 1)),
                end_l: None,
                expression_l: loc!(0, 7)
            })))
        );
    }

    #[test]
    fn test_string1_plain() {
        let mut parser = RustParser::new(b"'foo'");
        assert_eq!(
            parser.try_strings(),
            Ok(Box::new(Node::Str(Str {
                value: StringContent::from("foo"),
                begin_l: Some(loc!(0, 1)),
                end_l: Some(loc!(4, 5)),
                expression_l: loc!(0, 5)
            })))
        );
    }

    #[test]
    fn test_string1_interp() {
        let mut parser = RustParser::new(b"\"foo #{42} #@bar\"");
        assert_eq!(parser.try_strings(), Err(ParseError::empty()));
        todo!("implement me");
    }
}
