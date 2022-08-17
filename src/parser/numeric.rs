use crate::{
    builder::Builder,
    parser::{
        macros::{all_of, one_of},
        ParseResult, Parser,
    },
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn try_numeric(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "numeric",
            checkpoint = self.new_checkpoint(),
            {
                let (unary_t, number) = all_of!(
                    "-numeric",
                    self.try_token(TokenKind::tUMINUS_NUM),
                    self.try_simple_numeric(),
                )?;
                Ok(Builder::unary_num(unary_t, number, self.buffer()))
            },
            self.try_simple_numeric(),
        )
    }

    pub(crate) fn try_simple_numeric(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "simple numeric",
            {
                let integer_t = self.try_token(TokenKind::tINTEGER)?;
                Ok(Builder::integer(integer_t, self.buffer()))
            },
            {
                let float_t = self.try_token(TokenKind::tFLOAT)?;
                Ok(Builder::float(float_t, self.buffer()))
            },
            {
                let rational_t = self.try_token(TokenKind::tRATIONAL)?;
                Ok(Builder::rational(rational_t, self.buffer()))
            },
            {
                let complex_t = self.try_token(TokenKind::tIMAGINARY)?;
                Ok(Builder::complex(complex_t, self.buffer()))
            },
        )
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
