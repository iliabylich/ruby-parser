use crate::parser::ParseError;

pub(crate) struct OneOf<T, C> {
    pub(crate) name: &'static str,
    pub(crate) checkpoint: C,
    pub(crate) inner: Result<T, Vec<ParseError>>,
}

macro_rules! one_of {
    ($name:literal, checkpoint = $checkpoint:expr, $($branch:expr,)*) => {{
        use crate::parser::macros::one_of::OneOf;
        let mut chain = OneOf {
            name: $name,
            checkpoint: $checkpoint,
            inner: Err(vec![]),
        };

        let chain = one_of!(chain, [$($branch,)*]);

        match chain.inner {
            Ok(value) => Ok(value),
            Err(errors) => Err(crate::parser::ParseError::OneOfError {
                name: chain.name,
                variants: errors,
            })
        }
    }};

    ($name:literal, $($branch:expr,)*) => {{
        use crate::parser::macros::one_of::OneOf;
        let mut chain = OneOf {
            name: $name,
            checkpoint: crate::parser::checkpoint::NoCheckpoint,
            inner: Err(vec![]),
        };

        let chain = one_of!(chain, [$($branch,)*]);

        match chain.inner {
            Ok(value) => Ok(value),
            Err(errors) => Err(crate::parser::ParseError::OneOfError {
                name: chain.name,
                variants: errors,
            })
        }
    }};

    ($chain:expr, [$head:expr, $($tail:expr,)*]) => {{
        match &mut $chain.inner {
            Ok(_) => {
                $chain
            },
            Err(errors) => {
                match (|| { $head })() {
                    Ok(value) => {
                        $chain.inner = Ok(value);
                        $chain
                    },
                    Err(err) => {
                        errors.push(err);

                        // rollback
                        $chain.checkpoint.restore();

                        one_of!($chain, [ $($tail,)* ])
                    }
                }
            }
        }
    }};

    ($chain:expr, []) => { $chain }
}
pub(crate) use one_of;

#[cfg(test)]
mod tests {
    use crate::{
        parser::{ParseError, ParseResult, Parser},
        token::{Token, TokenKind},
        Loc,
    };

    fn parse(input: &[u8]) -> ParseResult<Token> {
        let mut parser = Parser::new(input);

        one_of!(
            "one_of_parser",
            checkpoint = parser.new_checkpoint(),
            parser.try_token(TokenKind::tINTEGER),
            parser.expect_token(TokenKind::tIVAR),
        )
    }

    #[test]
    fn test_one_of() {
        assert_eq!(
            parse(b"42"),
            Ok(Token {
                kind: TokenKind::tINTEGER,
                loc: Loc { start: 0, end: 2 },
                value: None
            })
        );

        assert_eq!(
            parse(b"@foo"),
            Ok(Token {
                kind: TokenKind::tIVAR,
                loc: Loc { start: 0, end: 4 },
                value: None
            })
        );
    }

    #[test]
    fn test_err() {
        assert_eq!(
            parse(b"foo"),
            Err(ParseError::OneOfError {
                name: "one_of_parser",
                variants: vec![
                    ParseError::TokenError {
                        lookahead: true,
                        expected: TokenKind::tINTEGER,
                        got: TokenKind::tIDENTIFIER,
                        loc: Loc { start: 0, end: 3 }
                    },
                    ParseError::TokenError {
                        lookahead: false,
                        expected: TokenKind::tIVAR,
                        got: TokenKind::tIDENTIFIER,
                        loc: Loc { start: 0, end: 3 }
                    },
                ]
            })
        )
    }
}
