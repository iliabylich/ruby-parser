use crate::{
    builder::Builder,
    parser::{macros::one_of, ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_keyword_variable(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "keyword variable",
            checkpoint = self.new_checkpoint(),
            parse_nil(self),
            parse_self(self),
            parse_true(self),
            parse_false(self),
            parse__file__(self),
            parse__line__(self),
            parse__encoding__(self),
        )
    }
}

fn parse_nil(parser: &mut Parser) -> ParseResult<Box<Node>> {
    parser
        .try_token(TokenKind::kNIL)
        .map(|nil_t| Builder::nil(nil_t))
}
fn parse_self(parser: &mut Parser) -> ParseResult<Box<Node>> {
    parser
        .try_token(TokenKind::kSELF)
        .map(|self_t| Builder::self_(self_t))
}
fn parse_true(parser: &mut Parser) -> ParseResult<Box<Node>> {
    parser
        .try_token(TokenKind::kTRUE)
        .map(|true_t| Builder::true_(true_t))
}
fn parse_false(parser: &mut Parser) -> ParseResult<Box<Node>> {
    parser
        .try_token(TokenKind::kFALSE)
        .map(|false_t| Builder::false_(false_t))
}
#[allow(non_snake_case)]
fn parse__file__(parser: &mut Parser) -> ParseResult<Box<Node>> {
    parser
        .try_token(TokenKind::k__FILE__)
        .map(|file_t| Builder::__file__(file_t))
}
#[allow(non_snake_case)]
fn parse__line__(parser: &mut Parser) -> ParseResult<Box<Node>> {
    parser
        .try_token(TokenKind::k__LINE__)
        .map(|line_t| Builder::__line__(line_t))
}
#[allow(non_snake_case)]
fn parse__encoding__(parser: &mut Parser) -> ParseResult<Box<Node>> {
    parser
        .try_token(TokenKind::k__ENCODING__)
        .map(|encoding_t| Builder::__encoding__(encoding_t))
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_nil() {
        assert_parses!(Parser::parse_keyword_variable, b"nil", "s(:nil)");
    }

    #[test]
    fn test_self() {
        assert_parses!(Parser::parse_keyword_variable, b"self", "s(:self)");
    }

    #[test]
    fn test_true() {
        assert_parses!(Parser::parse_keyword_variable, b"true", "s(:true)");
    }

    #[test]
    fn test_false() {
        assert_parses!(Parser::parse_keyword_variable, b"false", "s(:false)");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__file__() {
        assert_parses!(Parser::parse_keyword_variable, b"__FILE__", "s(:__FILE__)");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__line__() {
        assert_parses!(Parser::parse_keyword_variable, b"__LINE__", "s(:__LINE__)");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__encoding__() {
        assert_parses!(
            Parser::parse_keyword_variable,
            b"__ENCODING__",
            "s(:__ENCODING__)"
        );
    }
}
