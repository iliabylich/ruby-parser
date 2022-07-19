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
    pub(crate) fn try_keyword_variable(&mut self) -> Result<Box<Node>, ParseError> {
        self.one_of("keyword variable")
            .or_else(|| self.try_nil())
            .or_else(|| self.try_self())
            .or_else(|| self.try_true())
            .or_else(|| self.try_false())
            .or_else(|| self.try__file__())
            .or_else(|| self.try__line__())
            .or_else(|| self.try__encoding__())
            .done()
    }

    pub(crate) fn try_nil(&mut self) -> Result<Box<Node>, ParseError> {
        self.try_token(TokenKind::kNIL)
            .map(|nil_t| Builder::<C>::nil(nil_t))
    }
    pub(crate) fn try_self(&mut self) -> Result<Box<Node>, ParseError> {
        self.try_token(TokenKind::kSELF)
            .map(|self_t| Builder::<C>::self_(self_t))
    }
    pub(crate) fn try_true(&mut self) -> Result<Box<Node>, ParseError> {
        self.try_token(TokenKind::kTRUE)
            .map(|true_t| Builder::<C>::true_(true_t))
    }
    pub(crate) fn try_false(&mut self) -> Result<Box<Node>, ParseError> {
        self.try_token(TokenKind::kFALSE)
            .map(|false_t| Builder::<C>::false_(false_t))
    }
    #[allow(non_snake_case)]
    pub(crate) fn try__file__(&mut self) -> Result<Box<Node>, ParseError> {
        self.try_token(TokenKind::k__FILE__)
            .map(|file_t| Builder::<C>::__file__(file_t))
    }
    #[allow(non_snake_case)]
    pub(crate) fn try__line__(&mut self) -> Result<Box<Node>, ParseError> {
        self.try_token(TokenKind::k__LINE__)
            .map(|line_t| Builder::<C>::__line__(line_t))
    }
    #[allow(non_snake_case)]
    pub(crate) fn try__encoding__(&mut self) -> Result<Box<Node>, ParseError> {
        self.try_token(TokenKind::k__ENCODING__)
            .map(|encoding_t| Builder::<C>::__encoding__(encoding_t))
    }
}

#[test]
fn test_nil() {
    use crate::{loc::loc, nodes::Nil, RustParser};
    let mut parser = RustParser::new(b"nil");
    assert_eq!(
        parser.try_keyword_variable(),
        Ok(Box::new(Node::Nil(Nil {
            expression_l: loc!(0, 3)
        })))
    )
}

#[test]
fn test_self() {
    use crate::{loc::loc, nodes::Self_, RustParser};
    let mut parser = RustParser::new(b"self");
    assert_eq!(
        parser.try_keyword_variable(),
        Ok(Box::new(Node::Self_(Self_ {
            expression_l: loc!(0, 4)
        })))
    )
}

#[test]
fn test_true() {
    use crate::{loc::loc, nodes::True, RustParser};
    let mut parser = RustParser::new(b"true");
    assert_eq!(
        parser.try_keyword_variable(),
        Ok(Box::new(Node::True(True {
            expression_l: loc!(0, 4)
        })))
    )
}

#[test]
fn test_false() {
    use crate::{loc::loc, nodes::False, RustParser};
    let mut parser = RustParser::new(b"false");
    assert_eq!(
        parser.try_keyword_variable(),
        Ok(Box::new(Node::False(False {
            expression_l: loc!(0, 5)
        })))
    )
}

#[test]
#[allow(non_snake_case)]
fn test__file__() {
    use crate::{loc::loc, nodes::File, RustParser};
    let mut parser = RustParser::new(b"__FILE__");
    assert_eq!(
        parser.try_keyword_variable(),
        Ok(Box::new(Node::File(File {
            expression_l: loc!(0, 8)
        })))
    )
}

#[test]
#[allow(non_snake_case)]
fn test__line__() {
    use crate::{loc::loc, nodes::Line, RustParser};
    let mut parser = RustParser::new(b"__LINE__");
    assert_eq!(
        parser.try_keyword_variable(),
        Ok(Box::new(Node::Line(Line {
            expression_l: loc!(0, 8)
        })))
    )
}

#[test]
#[allow(non_snake_case)]
fn test__encoding__() {
    use crate::{loc::loc, nodes::Encoding, RustParser};
    let mut parser = RustParser::new(b"__ENCODING__");
    assert_eq!(
        parser.try_keyword_variable(),
        Ok(Box::new(Node::Encoding(Encoding {
            expression_l: loc!(0, 12)
        })))
    )
}
