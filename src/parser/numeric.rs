use crate::{
    builder::Builder,
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_numeric(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("numeric")
            .or_else(|| {
                let (uminus_num, simple_numeric) = self
                    .all_of("-numeric")
                    .and(|| self.parse_token(TokenKind::tUMINUS))
                    .and(|| self.parse_simple_numeric())
                    .stop()?;

                Ok(Builder::unary_num(
                    uminus_num,
                    simple_numeric,
                    self.buffer(),
                ))
            })
            .or_else(|| self.parse_simple_numeric())
            .stop()
    }

    pub(crate) fn parse_simple_numeric(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("simple numeric (without sign)")
            .or_else(|| {
                let integer_t = self.parse_token(TokenKind::tINTEGER)?;
                Ok(Builder::integer(integer_t, self.buffer()))
            })
            .or_else(|| {
                let float_t = self.parse_token(TokenKind::tFLOAT)?;
                Ok(Builder::float(float_t, self.buffer()))
            })
            .or_else(|| {
                let rational_t = self.parse_token(TokenKind::tRATIONAL)?;
                Ok(Builder::rational(rational_t, self.buffer()))
            })
            .or_else(|| {
                let imaginary_t = self.parse_token(TokenKind::tIMAGINARY)?;
                Ok(Builder::complex(imaginary_t, self.buffer()))
            })
            .stop()
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_integer() {
        assert_parses!(parse_numeric, b"42", "s(:int, \"42\")");
    }

    #[test]
    fn test_minus_integer() {
        assert_parses!(parse_numeric, b"-42", "s(:int, \"-42\")");
    }

    #[test]
    fn test_float() {
        assert_parses!(parse_numeric, b"4.2", "s(:float, \"4.2\")");
    }

    #[test]
    fn test_minus_float() {
        assert_parses!(parse_numeric, b"-4.2", "s(:float, \"-4.2\")");
    }

    #[test]
    fn test_rational() {
        assert_parses!(parse_numeric, b"42r", "s(:rational, \"42r\")");
    }

    #[test]
    fn test_minus_rational() {
        assert_parses!(parse_numeric, b"-42r", "s(:rational, \"-42r\")");
    }

    #[test]
    fn test_complex() {
        assert_parses!(parse_numeric, b"42i", "s(:complex, \"42i\")");
    }

    #[test]
    fn test_minus_complex() {
        assert_parses!(parse_numeric, b"-42i", "s(:complex, \"-42i\")");
    }
}
