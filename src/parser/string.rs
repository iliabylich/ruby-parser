use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::{Token, TokenKind},
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_strings(&mut self) -> Option<Box<Node<'a>>> {
        None.or_else(|| self.try_char())
            .or_else(|| self.try_string_seq())
    }

    fn try_char(&mut self) -> Option<Box<Node<'a>>> {
        if self.current_token().is(TokenKind::tCHAR) {
            let char_t = self.take_token();
            Some(Builder::<C>::character(char_t))
        } else {
            None
        }
    }

    fn try_string_seq(&mut self) -> Option<Box<Node<'a>>> {
        let mut parts = vec![];
        while let Some(string) = self.try_string1() {
            parts.push(*string);
        }
        if parts.is_empty() {
            None
        } else {
            Some(Builder::<C>::string_compose(None, parts, None))
        }
    }

    fn try_string1(&mut self) -> Option<Box<Node<'a>>> {
        let string_beg_t = None
            .or_else(|| self.try_token(TokenKind::tDSTRING_BEG))
            .or_else(|| self.try_token(TokenKind::tSTRING_BEG))
            .or_else(|| self.try_token(TokenKind::tHEREDOC_BEG))?;
        let string_contents = self.parse_string_contents();
        let string_end_t = self.expect_token(TokenKind::tSTRING_END);
        // TODO: dedent_heredoc
        Some(Builder::<C>::string_compose(
            Some(string_beg_t),
            string_contents,
            Some(string_end_t),
        ))
    }

    // This rule can be `none`
    pub(crate) fn parse_string_contents(&mut self) -> Vec<Node<'a>> {
        let mut strings = vec![];
        while let Some(string_content) = self.try_string_content() {
            strings.push(*string_content);
        }
        strings
    }

    pub(crate) fn try_string_content(&mut self) -> Option<Box<Node<'a>>> {
        match self.current_token().kind() {
            TokenKind::tSTRING_CONTENT => {
                let string_content_t = self.take_token();
                Some(Builder::<C>::string_internal(
                    string_content_t,
                    self.buffer(),
                ))
            }

            TokenKind::tSTRING_DVAR => {
                let string_dvar_t = self.take_token();
                if let Some(string_dvar) = self.try_string_dvar() {
                    panic!(
                        "tSTRING_DVAR string_dvar {:?} {:?}",
                        string_dvar_t, string_dvar
                    )
                } else {
                    panic!("expected string_dvar, got {:?}", self.current_token())
                }
            }

            TokenKind::tSTRING_DBEG => {
                let string_dbeg_t = self.take_token();
                let compstmt = self.try_compstmt();
                let string_dend_t = self.expect_token(TokenKind::tSTRING_DEND);
                panic!(
                    "tSTRING_DBEG compstmt tSTRING_DEND {:?} {:?} {:?}",
                    string_dbeg_t, compstmt, string_dend_t
                )
            }

            _ => None,
        }
    }

    fn try_string_dvar(&mut self) -> Option<Token> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{loc::loc, nodes::Str, string_content::StringContent, Node, RustParser};

    #[test]
    fn test_char() {
        let mut parser = RustParser::new(b"?\\u0001");
        assert_eq!(
            parser.try_strings(),
            Some(Box::new(Node::Str(Str {
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
            Some(Box::new(Node::Str(Str {
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
        assert_eq!(parser.try_strings(), None);
        todo!("implement me");
    }
}
