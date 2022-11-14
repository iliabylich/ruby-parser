use crate::{transactions::steps::Steps, Loc, TokenKind};

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum ParseError {
    TokenError {
        lookahead: bool,
        expected: TokenKind,
        got: TokenKind,
        loc: Loc,
    },

    OneOfError {
        name: &'static str,
        variants: Vec<ParseError>,
    },

    SeqError {
        name: &'static str,
        steps: Steps,
        error: Box<ParseError>,
    },
}
