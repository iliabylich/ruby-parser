use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::TokenValue,
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    fn parse_keyword_variable(&mut self) -> Option<Box<Node<'a>>> {
        None.or_else(|| self.parse_nil())
            .or_else(|| self.parse_self())
            .or_else(|| self.parse_true())
            .or_else(|| self.parse_false())
            .or_else(|| self.parse__file__())
            .or_else(|| self.parse__line__())
            .or_else(|| self.parse__encoding__())
    }

    fn parse_nil(&mut self) -> Option<Box<Node<'a>>> {
        self.try_token(TokenValue::kNIL)
            .map(|nil_t| Builder::<C>::nil(nil_t))
    }
    fn parse_self(&mut self) -> Option<Box<Node<'a>>> {
        self.try_token(TokenValue::kSELF)
            .map(|self_t| Builder::<C>::self_(self_t))
    }
    fn parse_true(&mut self) -> Option<Box<Node<'a>>> {
        self.try_token(TokenValue::kTRUE)
            .map(|true_t| Builder::<C>::true_(true_t))
    }
    fn parse_false(&mut self) -> Option<Box<Node<'a>>> {
        self.try_token(TokenValue::kFALSE)
            .map(|false_t| Builder::<C>::false_(false_t))
    }
    #[allow(non_snake_case)]
    fn parse__file__(&mut self) -> Option<Box<Node<'a>>> {
        self.try_token(TokenValue::k__FILE__)
            .map(|file_t| Builder::<C>::__file__(file_t))
    }
    #[allow(non_snake_case)]
    fn parse__line__(&mut self) -> Option<Box<Node<'a>>> {
        self.try_token(TokenValue::k__LINE__)
            .map(|line_t| Builder::<C>::__line__(line_t))
    }
    #[allow(non_snake_case)]
    fn parse__encoding__(&mut self) -> Option<Box<Node<'a>>> {
        self.try_token(TokenValue::k__ENCODING__)
            .map(|encoding_t| Builder::<C>::__encoding__(encoding_t))
    }
}

#[test]
fn test_nil() {
    use crate::{loc::loc, nodes::Nil, RustParser};
    let mut parser = RustParser::new(b"nil");
    assert_eq!(
        parser.parse_keyword_variable(),
        Some(Box::new(Node::Nil(Nil {
            expression_l: loc!(0, 3)
        })))
    )
}

#[test]
fn test_self() {
    use crate::{loc::loc, nodes::Self_, RustParser};
    let mut parser = RustParser::new(b"self");
    assert_eq!(
        parser.parse_keyword_variable(),
        Some(Box::new(Node::Self_(Self_ {
            expression_l: loc!(0, 4)
        })))
    )
}

#[test]
fn test_true() {
    use crate::{loc::loc, nodes::True, RustParser};
    let mut parser = RustParser::new(b"true");
    assert_eq!(
        parser.parse_keyword_variable(),
        Some(Box::new(Node::True(True {
            expression_l: loc!(0, 4)
        })))
    )
}

#[test]
fn test_false() {
    use crate::{loc::loc, nodes::False, RustParser};
    let mut parser = RustParser::new(b"false");
    assert_eq!(
        parser.parse_keyword_variable(),
        Some(Box::new(Node::False(False {
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
        parser.parse_keyword_variable(),
        Some(Box::new(Node::File(File {
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
        parser.parse_keyword_variable(),
        Some(Box::new(Node::Line(Line {
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
        parser.parse_keyword_variable(),
        Some(Box::new(Node::Encoding(Encoding {
            expression_l: loc!(0, 12)
        })))
    )
}
