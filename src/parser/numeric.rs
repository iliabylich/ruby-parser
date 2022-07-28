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
                let (uminus_num, simple_numeric) = self
                    .all_of("-numeric")
                    .and(|| self.try_token(TokenKind::tUMINUS))
                    .and(|| self.try_simple_numeric())
                    .stop()?;

                Ok(Builder::<C>::unary_num(
                    uminus_num,
                    simple_numeric,
                    self.buffer(),
                ))
            })
            .or_else(|| self.try_simple_numeric())
            .stop()
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
            .stop()
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_integer() {
        assert_parses!(try_numeric, b"42", "s(:int, \"42\")");
    }

    #[test]
    fn test_minus_integer() {
        assert_parses!(try_numeric, b"-42", "s(:int, \"-42\")");
    }

    #[test]
    fn test_float() {
        assert_parses!(try_numeric, b"4.2", "s(:float, \"4.2\")");
    }

    #[test]
    fn test_minus_float() {
        assert_parses!(try_numeric, b"-4.2", "s(:float, \"-4.2\")");
    }

    #[test]
    fn test_rational() {
        assert_parses!(try_numeric, b"42r", "s(:rational, \"42r\")");
    }

    #[test]
    fn test_minus_rational() {
        assert_parses!(try_numeric, b"-42r", "s(:rational, \"-42r\")");
    }

    #[test]
    fn test_complex() {
        assert_parses!(try_numeric, b"42i", "s(:complex, \"42i\")");
    }

    #[test]
    fn test_minus_complex() {
        assert_parses!(try_numeric, b"-42i", "s(:complex, \"-42i\")");
    }
}
