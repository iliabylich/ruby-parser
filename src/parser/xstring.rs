use crate::{
    builder::{Builder, Constructor},
    lexer::strings::{
        literal::StringLiteral,
        types::{Interpolation, StringInterp},
    },
    parser::Parser,
    token::{Token, TokenValue},
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_xstring(&mut self) -> Option<Box<Node<'a>>> {
        let xstring_beg_t = None
            .or_else(|| self.read_backtick_identifier_as_xstring_beg())
            .or_else(|| self.try_token(TokenValue::tXHEREDOC_BEG))?;

        // now we need to manually push a xstring literal
        // Lexer is not capable of doing it
        self.lexer
            .string_literals
            .push(StringLiteral::StringInterp(StringInterp::new(
                Interpolation::new(self.lexer.curly_nest),
                b'`',
                b'`',
            )));

        let xstring_contents = self.parse_xstring_contents();
        let string_end_t = self.expect_token(TokenValue::tSTRING_END);
        Some(Builder::<C>::xstring_compose(
            xstring_beg_t,
            xstring_contents,
            string_end_t,
        ))
    }

    fn parse_xstring_contents(&mut self) -> Vec<Node<'a>> {
        let mut contents = vec![];
        while let Some(content) = self.try_string_content() {
            contents.push(*content)
        }
        contents
    }

    fn read_backtick_identifier_as_xstring_beg(&mut self) -> Option<Token<'a>> {
        let loc = self.current_token().loc();
        if self.current_token().is(TokenValue::tIDENTIFIER) {
            if self.buffer().slice(loc.start, loc.end) == Some(b"`") {
                self.take_token();
                return Some(Token(TokenValue::tXSTRING_BEG, loc));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{loc::loc, string_content::StringContent, Node, RustParser};

    #[test]
    fn test_xstring_plain() {
        use crate::nodes::{Str, Xstr};

        let mut parser = RustParser::new(b"`foo`");
        assert_eq!(
            parser.try_xstring(),
            Some(Box::new(Node::Xstr(Xstr {
                parts: vec![Node::Str(Str {
                    value: StringContent::from("foo"),
                    begin_l: None,
                    end_l: None,
                    expression_l: loc!(1, 4)
                })],
                begin_l: loc!(0, 1),
                end_l: loc!(4, 5),
                expression_l: loc!(0, 5)
            })))
        );
    }
}