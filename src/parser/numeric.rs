use crate::{
    builder::Builder,
    parser::{ParseError, ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn try_numeric(&mut self) -> ParseResult<Box<Node>> {
        match self.current_token() {
            unary_t @ Token {
                kind: TokenKind::tUMINUS_NUM,
                ..
            } => {
                self.skip_token();

                match self.try_simple_numeric() {
                    Ok(simple_numeric) => {
                        Ok(Builder::unary_num(unary_t, simple_numeric, self.buffer()))
                    }
                    Err(error) => Err(ParseError::seq_error("-<number>", unary_t, error)),
                }
            }
            _ => self.try_simple_numeric(),
        }
    }

    pub(crate) fn try_simple_numeric(&mut self) -> ParseResult<Box<Node>> {
        let token = self.current_token();
        match token.kind {
            TokenKind::tINTEGER => {
                self.skip_token();
                Ok(Builder::integer(token, self.buffer()))
            }

            TokenKind::tFLOAT => {
                self.skip_token();
                Ok(Builder::float(token, self.buffer()))
            }

            TokenKind::tRATIONAL => {
                self.skip_token();
                Ok(Builder::rational(token, self.buffer()))
            }

            TokenKind::tIMAGINARY => {
                self.skip_token();
                Ok(Builder::complex(token, self.buffer()))
            }

            got => Err(ParseError::OneOfError {
                name: "simple numeric",
                variants: vec![
                    ParseError::TokenError {
                        lookahead: true,
                        expected: TokenKind::tINTEGER,
                        got,
                        loc: token.loc,
                    },
                    ParseError::TokenError {
                        lookahead: true,
                        expected: TokenKind::tFLOAT,
                        got,
                        loc: token.loc,
                    },
                    ParseError::TokenError {
                        lookahead: true,
                        expected: TokenKind::tRATIONAL,
                        got,
                        loc: token.loc,
                    },
                    ParseError::TokenError {
                        lookahead: true,
                        expected: TokenKind::tIMAGINARY,
                        got,
                        loc: token.loc,
                    },
                ],
            }),
        }
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
