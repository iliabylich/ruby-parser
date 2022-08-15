macro_rules! at_least_once {
    ($iter:expr, checkpoint = $checkpoint:expr) => {
        (|| {
            let mut items = vec![];

            let item = $iter?;
            items.push(item);

            loop {
                let checkpoint = $checkpoint;

                match $iter {
                    Ok(item) => items.push(item),
                    Err(_err) => {
                        checkpoint.restore();
                        break;
                    }
                }
            }

            Ok(items)
        })()
    };
}
pub(crate) use at_least_once;

#[cfg(test)]
mod tests {
    use crate::{
        loc::loc,
        parser::{ParseError, ParseResult, Parser},
        token::{token, Token, TokenKind},
    };

    fn parse(input: &[u8]) -> (Parser, ParseResult<Vec<Token>>) {
        let mut parser = Parser::new(input);

        let ints = at_least_once!(
            parser.try_token(TokenKind::tINTEGER),
            checkpoint = parser.new_checkpoint()
        );

        (parser, ints)
    }

    #[test]
    fn test_ok() {
        let (_, result) = parse(b"1 2 3");
        assert_eq!(
            result,
            Ok(vec![
                token!(tINTEGER, loc!(0, 1)),
                token!(tINTEGER, loc!(2, 3)),
                token!(tINTEGER, loc!(4, 5)),
            ])
        );
    }

    #[test]
    fn test_nothing() {
        let (_, result) = parse(b"");
        assert_eq!(
            result,
            Err(ParseError::TokenError {
                lookahead: true,
                expected: TokenKind::tINTEGER,
                got: TokenKind::tEOF,
                loc: loc!(0, 0)
            })
        )
    }

    #[test]
    fn test_stop_other_item() {
        let (mut parser, result) = parse(b"1 2 foo");
        assert_eq!(
            result,
            Ok(vec![
                token!(tINTEGER, loc!(0, 1)),
                token!(tINTEGER, loc!(2, 3)),
            ])
        );

        assert_eq!(parser.current_token(), token!(tIDENTIFIER, loc!(4, 7)))
    }
}
