use crate::{
    builder::Builder,
    parser::{
        base::{at_most_one_is_true, ExactToken, Maybe1, Rule, SeparatedBy},
        Value,
    },
    Node, Parser, Token, TokenKind,
};

pub(crate) struct ParenArgs;
impl Rule for ParenArgs {
    type Output = (Token, Vec<Node>, Token);

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tLPAREN)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        todo!()
    }
}

pub(crate) struct Args;
impl Rule for Args {
    type Output = Vec<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        Arglist::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let arglist = Arglist::parse(parser);
        // TODO: validate that it has exactly 1 element if it's a command
        arglist
    }
}

pub(crate) struct CallArgs;
impl Rule for CallArgs {
    type Output = (Option<Token>, Vec<Node>, Option<Token>);

    fn starts_now(parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if Args::starts_now(parser) {
            let args = Args::parse(parser);
            (None, args, None)
        } else if ParenArgs::starts_now(parser) {
            let (lparen_t, args, rparen_t) = ParenArgs::parse(parser);
            (Some(lparen_t), args, Some(rparen_t))
        } else {
            (None, vec![], None)
        }
    }
}

pub(crate) struct Mrhs;
impl Rule for Mrhs {
    type Output = Vec<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        Mrhs1::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        type CommaT = ExactToken<{ TokenKind::tCOMMA as u8 }>;
        type R = SeparatedBy<Mrhs1, CommaT>;
        let (items, _commas) = R::parse(parser);
        items
    }
}

struct Mrhs1;
impl Rule for Mrhs1 {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        at_most_one_is_true([
            parser.current_token().is(TokenKind::tSTAR),
            Value::starts_now(parser),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if parser.current_token().is(TokenKind::tSTAR) {
            let star_t = parser.take_token();
            let value = Value::parse(parser);
            Builder::splat(star_t, value)
        } else if Value::starts_now(parser) {
            Value::parse(parser)
        } else {
            unreachable!()
        }
    }
}

struct Arglist;
impl Rule for Arglist {
    type Output = Vec<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        Arg::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let (args, _commas) =
            SeparatedBy::<Arg, ExactToken<{ TokenKind::tCOMMA as u8 }>>::parse(parser);
        args
    }
}

struct Arg;
impl Rule for Arg {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        let token = parser.current_token();
        at_most_one_is_true([
            Value::starts_now(parser),
            token.is(TokenKind::tLABEL),
            token.is(TokenKind::tSTAR),
            token.is(TokenKind::tDSTAR),
            token.is(TokenKind::tAMPER),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if Value::starts_now(parser) {
            let value = Value::parse(parser);
            if parser.current_token().is(TokenKind::tASSOC) {
                let key = value;
                let assoc_t = parser.take_token();
                let value = Value::parse(parser);
                Builder::pair(key, assoc_t, value)
            } else if parser.current_token().is(TokenKind::tCOLON)
            /* TODO: && key is a string */
            {
                let key = value;
                let colon_t = parser.take_token();
                let value = Value::parse(parser);
                Builder::pair_quoted(key, colon_t, value)
            } else {
                value
            }
        } else if parser.current_token().is(TokenKind::tLABEL) {
            todo!()
        } else if parser.current_token().is(TokenKind::tSTAR) {
            todo!()
        } else if parser.current_token().is(TokenKind::tDSTAR) {
            todo!()
        } else if parser.current_token().is(TokenKind::tAMPER) {
            todo!()
        } else {
            unreachable!()
        }
    }
}
