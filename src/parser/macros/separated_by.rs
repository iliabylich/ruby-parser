macro_rules! separated_by {
    ($name:literal, checkpoint = $checkpoint:expr, item = $item:expr, sep = $sep:expr) => {
        (|| {
            let mut items = vec![];
            let mut separators = vec![];

            let first = $item?;
            items.push(first);

            loop {
                let checkpoint = $checkpoint;

                match $sep {
                    Ok(sep) => separators.push(sep),
                    Err(_err) => break,
                }

                match $item {
                    Ok(item) => items.push(item),
                    Err(_err) => {
                        checkpoint.restore();
                        break;
                    }
                }
            }

            Ok((items, separators))
        })()
    };
}
pub(crate) use separated_by;

#[cfg(test)]
mod tests {
    use crate::{
        loc::loc,
        parser::{ParseError, ParseResult, Parser},
        token::{token, Token, TokenKind},
    };

    fn parse(input: &[u8]) -> (Parser, ParseResult<Vec<Token>>) {
        let mut parser = Parser::new(input);

        let ints = separated_by!(
            "ints separated by comma",
            checkpoint = parser.new_checkpoint(),
            item = parser.try_token(TokenKind::tINTEGER),
            sep = parser.try_token(TokenKind::tCOMMA)
        )
        .map(|(items, _separators)| items);

        (parser, ints)
    }

    #[test]
    fn test_ok() {
        let (_, result) = parse(b"1, 2, 3");
        assert_eq!(
            result,
            Ok(vec![
                token!(tINTEGER, loc!(0, 1)),
                token!(tINTEGER, loc!(3, 4)),
                token!(tINTEGER, loc!(6, 7)),
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
        let (mut parser, result) = parse(b"1, 2, foo");
        assert_eq!(
            result,
            Ok(vec![
                token!(tINTEGER, loc!(0, 1)),
                token!(tINTEGER, loc!(3, 4)),
            ])
        );

        assert_eq!(parser.current_token(), token!(tCOMMA, loc!(4, 5)))
    }

    #[test]
    fn test_stop_no_sep() {
        let (mut parser, result) = parse(b"1, 2 + foo");
        assert_eq!(
            result,
            Ok(vec![
                token!(tINTEGER, loc!(0, 1)),
                token!(tINTEGER, loc!(3, 4)),
            ])
        );

        assert_eq!(parser.current_token(), token!(tPLUS, loc!(5, 6)))
    }
}
