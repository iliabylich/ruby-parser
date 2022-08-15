macro_rules! repeat2 {
    ($name:literal, $first:expr, $second:expr) => {
        (|| {
            let mut first = vec![];
            let mut second = vec![];

            loop {
                match (|| $first)() {
                    Ok(value) => first.push(value),
                    Err(err) => break,
                }

                match (|| $second)() {
                    Ok(value) => second.push(value),
                    Err(error) => match error.strip_lookaheads() {
                        Some(error) => {
                            return Err(crate::parser::ParseError::seq_error(
                                $name,
                                (first, second),
                                error,
                            ));
                        }
                        None => break,
                    },
                }
            }

            Ok((first, second))
        })()
    };
}
pub(crate) use repeat2;

#[cfg(test)]
mod tests {
    use crate::{
        loc::{loc, Loc},
        parser::{ParseError, ParseResult, Parser},
        token::{token, Token, TokenKind},
    };

    fn parse(input: &[u8]) -> ParseResult<Vec<Token>> {
        let mut parser = Parser::new(input);

        let (_commas, ints) = repeat2!(
            "[tCOMMA int]*",
            parser.try_token(TokenKind::tCOMMA),
            parser.expect_token(TokenKind::tINTEGER)
        )?;

        Ok(ints)
    }

    #[test]
    fn test_ok() {
        assert_eq!(
            parse(b", 1, 2, 3"),
            Ok(vec![
                Token {
                    kind: TokenKind::tINTEGER,
                    loc: Loc { start: 2, end: 3 },
                    value: None
                },
                Token {
                    kind: TokenKind::tINTEGER,
                    loc: Loc { start: 5, end: 6 },
                    value: None
                },
                Token {
                    kind: TokenKind::tINTEGER,
                    loc: Loc { start: 8, end: 9 },
                    value: None
                },
            ])
        );

        assert_eq!(parse(b""), Ok(vec![]))
    }

    #[test]
    fn test_err() {
        assert_eq!(
            parse(b", 1, 2, foo"),
            Err(ParseError::seq_error(
                "[tCOMMA int]*",
                vec![
                    token!(tCOMMA, loc!(0, 1)),
                    token!(tCOMMA, loc!(3, 4)),
                    token!(tCOMMA, loc!(6, 7)),
                    token!(tINTEGER, loc!(2, 3)),
                    token!(tINTEGER, loc!(5, 6)),
                ],
                ParseError::TokenError {
                    lookahead: false,
                    expected: TokenKind::tINTEGER,
                    got: TokenKind::tIDENTIFIER,
                    loc: Loc { start: 8, end: 11 }
                }
            ))
        )
    }
}
