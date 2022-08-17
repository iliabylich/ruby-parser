macro_rules! at_least_once {
    ($name:literal, $iter:expr) => {
        (|| {
            let mut items = vec![];

            let item = $iter?;
            items.push(*item);

            loop {
                match $iter {
                    Ok(item) => items.push(*item),
                    Err(error) => match error.strip_lookaheads() {
                        Some(error) => {
                            return Err($crate::parser::ParseError::seq_error($name, items, error))
                        }
                        None => {
                            break;
                        }
                    },
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

        let ints = at_least_once!("ints", parser.try_token(TokenKind::tINTEGER).map(Box::new));

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
