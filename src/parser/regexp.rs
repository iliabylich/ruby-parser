use crate::{
    builder::{Builder, Constructor},
    lexer::strings::{literal::StringLiteral, types::Regexp},
    loc::loc,
    parser::Parser,
    token::{token, Token, TokenKind},
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_regexp(&mut self) -> Option<Box<Node>> {
        let begin_t = self.try_token(TokenKind::tREGEXP_BEG).or_else(|| {
            let token = self.read_div_as_heredoc_beg()?;

            // now we need to manually push a xstring literal
            // Lexer is not capable of doing it
            self.lexer
                .string_literals
                .push(StringLiteral::Regexp(Regexp::new(
                    b'/',
                    b'/',
                    self.lexer.curly_nest,
                )));

            Some(token)
        })?;

        let contents = self.try_regexp_contents();
        let end_t = self.expect_token(TokenKind::tSTRING_END);

        let options = Builder::<C>::regexp_options(&end_t, self.buffer());
        Some(Builder::<C>::regexp_compose(
            begin_t, contents, end_t, options,
        ))
    }

    // This rule can be `none`
    fn try_regexp_contents(&mut self) -> Vec<Node> {
        let mut strings = vec![];
        while let Some(string_content) = self.try_string_content() {
            strings.push(*string_content);
        }
        strings
    }

    fn read_div_as_heredoc_beg(&mut self) -> Option<Token> {
        let loc = self.current_token().loc();
        if self.current_token().is(TokenKind::tDIVIDE) {
            self.take_token();
            Some(token!(TokenKind::tREGEXP_BEG, loc!(loc.start, loc.end)))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        loc::loc,
        nodes::{RegOpt, Regexp, Str},
        string_content::StringContent,
        Node, RustParser,
    };

    #[test]
    fn test_regexp() {
        let mut parser = RustParser::new(b"/foo/");
        assert_eq!(
            parser.try_regexp(),
            Some(Box::new(Node::Regexp(Regexp {
                parts: vec![Node::Str(Str {
                    value: StringContent::from("foo"),
                    begin_l: None,
                    end_l: None,
                    expression_l: loc!(1, 4)
                })],
                options: None,
                begin_l: loc!(0, 1),
                end_l: loc!(4, 5),
                expression_l: loc!(0, 5)
            })))
        );
    }

    #[test]
    fn test_regexp_with_options() {
        let mut parser = RustParser::new(b"/foo/mix");
        assert_eq!(
            parser.try_regexp(),
            Some(Box::new(Node::Regexp(Regexp {
                parts: vec![Node::Str(Str {
                    value: StringContent::from("foo"),
                    begin_l: None,
                    end_l: None,
                    expression_l: loc!(1, 4)
                })],
                options: Some(Box::new(Node::RegOpt(RegOpt {
                    options: Some(StringContent::from("imx")),
                    expression_l: loc!(5, 8)
                }))),
                begin_l: loc!(0, 1),
                end_l: loc!(4, 5),
                expression_l: loc!(0, 8)
            })))
        );
    }

    #[test]
    fn test_regexp_percent() {
        let mut parser = RustParser::new(b"%r{foo}");
        assert_eq!(
            parser.try_regexp(),
            Some(Box::new(Node::Regexp(Regexp {
                parts: vec![Node::Str(Str {
                    value: StringContent::from("foo"),
                    begin_l: None,
                    end_l: None,
                    expression_l: loc!(3, 6)
                })],
                options: None,
                begin_l: loc!(0, 3),
                end_l: loc!(6, 7),
                expression_l: loc!(0, 7)
            })))
        );
    }
}
