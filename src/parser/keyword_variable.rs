use crate::{
    builder::{Builder, Constructor},
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_keyword_variable(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("keyword variable")
            .or_else(|| self.try_nil())
            .or_else(|| self.try_self())
            .or_else(|| self.try_true())
            .or_else(|| self.try_false())
            .or_else(|| self.try__file__())
            .or_else(|| self.try__line__())
            .or_else(|| self.try__encoding__())
            .stop()
    }

    fn try_nil(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::kNIL)
            .map(|nil_t| Builder::<C>::nil(nil_t))
    }
    fn try_self(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::kSELF)
            .map(|self_t| Builder::<C>::self_(self_t))
    }
    fn try_true(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::kTRUE)
            .map(|true_t| Builder::<C>::true_(true_t))
    }
    fn try_false(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::kFALSE)
            .map(|false_t| Builder::<C>::false_(false_t))
    }
    #[allow(non_snake_case)]
    fn try__file__(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::k__FILE__)
            .map(|file_t| Builder::<C>::__file__(file_t))
    }
    #[allow(non_snake_case)]
    fn try__line__(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::k__LINE__)
            .map(|line_t| Builder::<C>::__line__(line_t))
    }
    #[allow(non_snake_case)]
    fn try__encoding__(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::k__ENCODING__)
            .map(|encoding_t| Builder::<C>::__encoding__(encoding_t))
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_nil() {
        assert_parses!(try_keyword_variable, b"nil", "s(:nil)");
    }

    #[test]
    fn test_self() {
        assert_parses!(try_keyword_variable, b"self", "s(:self)");
    }

    #[test]
    fn test_true() {
        assert_parses!(try_keyword_variable, b"true", "s(:true)");
    }

    #[test]
    fn test_false() {
        assert_parses!(try_keyword_variable, b"false", "s(:false)");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__file__() {
        assert_parses!(try_keyword_variable, b"__FILE__", "s(:__FILE__)");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__line__() {
        assert_parses!(try_keyword_variable, b"__LINE__", "s(:__LINE__)");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__encoding__() {
        assert_parses!(try_keyword_variable, b"__ENCODING__", "s(:__ENCODING__)");
    }
}
