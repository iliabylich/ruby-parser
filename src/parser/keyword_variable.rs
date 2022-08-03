use crate::{
    builder::Builder,
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_keyword_variable(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("keyword variable")
            .or_else(|| self.parse_nil())
            .or_else(|| self.parse_self())
            .or_else(|| self.parse_true())
            .or_else(|| self.parse_false())
            .or_else(|| self.parse__file__())
            .or_else(|| self.parse__line__())
            .or_else(|| self.parse__encoding__())
            .stop()
    }

    fn parse_nil(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::kNIL)
            .map(|nil_t| Builder::nil(nil_t))
    }
    fn parse_self(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::kSELF)
            .map(|self_t| Builder::self_(self_t))
    }
    fn parse_true(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::kTRUE)
            .map(|true_t| Builder::true_(true_t))
    }
    fn parse_false(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::kFALSE)
            .map(|false_t| Builder::false_(false_t))
    }
    #[allow(non_snake_case)]
    fn parse__file__(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::k__FILE__)
            .map(|file_t| Builder::__file__(file_t))
    }
    #[allow(non_snake_case)]
    fn parse__line__(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::k__LINE__)
            .map(|line_t| Builder::__line__(line_t))
    }
    #[allow(non_snake_case)]
    fn parse__encoding__(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::k__ENCODING__)
            .map(|encoding_t| Builder::__encoding__(encoding_t))
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_nil() {
        assert_parses!(parse_keyword_variable, b"nil", "s(:nil)");
    }

    #[test]
    fn test_self() {
        assert_parses!(parse_keyword_variable, b"self", "s(:self)");
    }

    #[test]
    fn test_true() {
        assert_parses!(parse_keyword_variable, b"true", "s(:true)");
    }

    #[test]
    fn test_false() {
        assert_parses!(parse_keyword_variable, b"false", "s(:false)");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__file__() {
        assert_parses!(parse_keyword_variable, b"__FILE__", "s(:__FILE__)");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__line__() {
        assert_parses!(parse_keyword_variable, b"__LINE__", "s(:__LINE__)");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__encoding__() {
        assert_parses!(parse_keyword_variable, b"__ENCODING__", "s(:__ENCODING__)");
    }
}
