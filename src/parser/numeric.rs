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
    pub(crate) fn try_numeric(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("numeric")
            .or_else(|| {
                let uminus_num = self.try_token(TokenKind::tUMINUS)?;
                // If there's no number after `-` is still could be `-expr`,
                // that's fine, here we handle only numeric literals
                let simple_numeric = self.try_simple_numeric()?;
                Ok(Builder::<C>::unary_num(
                    uminus_num,
                    simple_numeric,
                    self.buffer(),
                ))
            })
            .or_else(|| self.try_simple_numeric())
            .unwrap()
    }

    pub(crate) fn try_simple_numeric(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("simple numeric (without sign)")
            .or_else(|| {
                let integer_t = self.try_token(TokenKind::tINTEGER)?;
                Ok(Builder::<C>::integer(integer_t, self.buffer()))
            })
            .or_else(|| {
                let float_t = self.try_token(TokenKind::tFLOAT)?;
                Ok(Builder::<C>::float(float_t, self.buffer()))
            })
            .or_else(|| {
                let rational_t = self.try_token(TokenKind::tRATIONAL)?;
                Ok(Builder::<C>::rational(rational_t, self.buffer()))
            })
            .or_else(|| {
                let imaginary_t = self.try_token(TokenKind::tIMAGINARY)?;
                Ok(Builder::<C>::complex(imaginary_t, self.buffer()))
            })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{loc::loc, string_content::StringContent, Node, RustParser};

    #[test]
    fn test_integer() {
        use crate::nodes::Int;
        let mut parser = RustParser::new(b"42");
        assert_eq!(
            parser.try_numeric(),
            Ok(Box::new(Node::Int(Int {
                value: StringContent::from("42"),
                operator_l: None,
                expression_l: loc!(0, 2)
            })))
        );
    }

    #[test]
    fn test_minus_integer() {
        use crate::nodes::Int;
        let mut parser = RustParser::new(b"-42");
        assert_eq!(
            parser.try_numeric(),
            Ok(Box::new(Node::Int(Int {
                value: StringContent::from("-42"),
                operator_l: Some(loc!(0, 1)),
                expression_l: loc!(0, 3)
            })))
        );
    }

    #[test]
    fn test_float() {
        use crate::nodes::Float;
        let mut parser = RustParser::new(b"4.2");
        assert_eq!(
            parser.try_numeric(),
            Ok(Box::new(Node::Float(Float {
                value: StringContent::from("4.2"),
                operator_l: None,
                expression_l: loc!(0, 3)
            })))
        );
    }

    #[test]
    fn test_minus_float() {
        use crate::nodes::Float;
        let mut parser = RustParser::new(b"-4.2");
        assert_eq!(
            parser.try_numeric(),
            Ok(Box::new(Node::Float(Float {
                value: StringContent::from("-4.2"),
                operator_l: Some(loc!(0, 1)),
                expression_l: loc!(0, 4)
            })))
        );
    }

    #[test]
    fn test_rational() {
        use crate::nodes::Rational;
        let mut parser = RustParser::new(b"42r");
        assert_eq!(
            parser.try_numeric(),
            Ok(Box::new(Node::Rational(Rational {
                value: StringContent::from("42r"),
                operator_l: None,
                expression_l: loc!(0, 3)
            })))
        );
    }

    #[test]
    fn test_minus_rational() {
        use crate::nodes::Rational;
        let mut parser = RustParser::new(b"-42r");
        assert_eq!(
            parser.try_numeric(),
            Ok(Box::new(Node::Rational(Rational {
                value: StringContent::from("-42r"),
                operator_l: Some(loc!(0, 1)),
                expression_l: loc!(0, 4)
            })))
        );
    }

    #[test]
    fn test_complex() {
        use crate::nodes::Complex;
        let mut parser = RustParser::new(b"42i");
        assert_eq!(
            parser.try_numeric(),
            Ok(Box::new(Node::Complex(Complex {
                value: StringContent::from("42i"),
                operator_l: None,
                expression_l: loc!(0, 3)
            })))
        );
    }

    #[test]
    fn test_minus_complex() {
        use crate::nodes::Complex;
        let mut parser = RustParser::new(b"-42i");
        assert_eq!(
            parser.try_numeric(),
            Ok(Box::new(Node::Complex(Complex {
                value: StringContent::from("-42i"),
                operator_l: Some(loc!(0, 1)),
                expression_l: loc!(0, 4)
            })))
        );
    }
}
