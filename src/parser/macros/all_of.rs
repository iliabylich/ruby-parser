macro_rules! all_of {
    ($name:literal, $head:expr, $($tail:expr,)*) => {{
        all_of!(
            name = $name;
            values = ();
            pool = (_a, _b, _c, _d, _e, _f, _g, _h, _i,);
            [ $head, $($tail,)* ]
        )
    }};

    (
        name = $name:literal;
        values = ($($values:ident,)*);
        pool = ($poolvalue:ident, $($pooltail:ident,)*);
        [ $head:expr, $($tail:expr,)* ]
    ) => {
        (|$($values,)*| {
            match $head {
                Ok($poolvalue) => {
                    all_of!(
                        name = $name;
                        values = ($($values,)* $poolvalue,);
                        pool = ($($pooltail,)*);
                        [ $($tail,)* ]
                    )
                }
                Err(error) => {
                    return Err($crate::parser::ParseError::seq_error($name, ($($values,)*), error))
                }
            }
        })($($values,)*)
    };

    (
        name = $name:literal;
        values = ($($values:ident,)*);
        pool = ($poolvalue:ident, $($pooltail:ident,)*);
        []
    ) => {
        Ok(( $($values,)* ))
    };
}
pub(crate) use all_of;

#[cfg(test)]
mod tests {
    use crate::{
        parser::{ParseError, ParseResult, Parser},
        token::{Token, TokenKind},
        Loc,
    };

    fn parse(input: &[u8]) -> ParseResult<(Token, Token)> {
        let mut parser = Parser::new(input);

        let (lbrack_t, rbrack_t) = all_of!(
            "foo",
            parser.try_token(TokenKind::tLBRACK),
            parser.try_token(TokenKind::tRBRACK),
        )?;

        Ok((lbrack_t, rbrack_t))
    }

    #[test]
    fn test_ok() {
        assert_eq!(
            parse(b"[ ]"),
            Ok((
                Token {
                    kind: TokenKind::tLBRACK,
                    loc: Loc { start: 0, end: 1 },
                    value: None
                },
                Token {
                    kind: TokenKind::tRBRACK,
                    loc: Loc { start: 2, end: 3 },
                    value: None
                },
            ))
        )
    }

    #[test]
    fn test_err() {
        assert_eq!(
            parse(b"42"),
            Err(ParseError::seq_error(
                "foo",
                (),
                ParseError::TokenError {
                    lookahead: true,
                    expected: TokenKind::tLBRACK,
                    got: TokenKind::tINTEGER,
                    loc: Loc { start: 0, end: 2 }
                }
            ))
        )
    }
}