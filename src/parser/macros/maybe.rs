macro_rules! maybe {
    ($rule:expr) => {
        match $rule {
            Ok(value) => Ok(Some(value)),
            Err(error) => match error.strip_lookaheads() {
                Some(error) => Err(error),
                None => Ok(None),
            },
        }
    };
}
pub(crate) use maybe;

#[cfg(test)]
mod tests {
    use crate::{
        parser::{macros::all_of, ParseError, ParseResult, Parser},
        token::{Token, TokenKind},
        Loc,
    };

    fn parse(input: &[u8]) -> ParseResult<(Option<Token>, Token)> {
        let mut parser = Parser::new(input);

        all_of!(
            "[int] float",
            maybe!(parser.try_token(TokenKind::tINTEGER)),
            parser.try_token(TokenKind::tFLOAT),
        )
    }

    #[test]
    fn test_ok() {
        assert_eq!(
            parse(b"42 42.5"),
            Ok((
                Some(Token {
                    kind: TokenKind::tINTEGER,
                    loc: Loc { start: 0, end: 2 },
                    value: None
                }),
                Token {
                    kind: TokenKind::tFLOAT,
                    loc: Loc { start: 3, end: 7 },
                    value: None
                }
            ))
        );

        assert_eq!(
            parse(b"42.5"),
            Ok((
                None,
                Token {
                    kind: TokenKind::tFLOAT,
                    loc: Loc { start: 0, end: 4 },
                    value: None
                }
            ))
        );
    }

    #[test]
    fn test_err() {
        assert_eq!(
            parse(b"foo"),
            Err(ParseError::seq_error(
                "[int] float",
                (),
                ParseError::TokenError {
                    lookahead: true,
                    expected: TokenKind::tFLOAT,
                    got: TokenKind::tIDENTIFIER,
                    loc: Loc { start: 0, end: 3 }
                }
            ))
        );

        assert_eq!(
            parse(b"42 foo"),
            Err(ParseError::seq_error(
                "[int] float",
                (Token {
                    kind: TokenKind::tINTEGER,
                    loc: Loc { start: 0, end: 2 },
                    value: None
                }),
                ParseError::TokenError {
                    lookahead: true,
                    expected: TokenKind::tFLOAT,
                    got: TokenKind::tIDENTIFIER,
                    loc: Loc { start: 3, end: 6 }
                }
            ))
        )
    }
}
