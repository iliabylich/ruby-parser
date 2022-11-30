use crate::{
    builder::Builder,
    parser::{
        arg::ArgType,
        base::{ParseResult, Rule},
        Arg, EndlessMethodDef, Primary,
    },
    Node, Parser, Token, TokenKind,
};

pub(crate) struct Value<T>
where
    T: ValueType,
{
    _t: std::marker::PhantomData<T>,
}
impl<T> Rule for Value<T>
where
    T: ValueType,
{
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        T::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        T::parse(parser)
    }
}

pub(crate) trait ValueType {
    // Prefix operators
    fn prefix_operator_power(token: Token) -> Option<(u8, u8)>;
    fn build_prefix_op(op_t: Token, arg: Box<Node>, parser: &mut Parser) -> ParseResult<Box<Node>>;

    // Binary operators
    fn binary_operator_power(token: Token) -> Option<(u8, u8)>;
    fn build_binary_op(
        op_t: Token,
        lhs: Box<Node>,
        parser: &mut Parser,
        r_bp: u8,
    ) -> ParseResult<Box<Node>>;

    // Postfix operators
    fn postfix_operator_power(token: Token) -> Option<(u8, u8)>;
    fn build_postfix_op(op_t: Token, arg: Box<Node>, parser: &mut Parser)
        -> ParseResult<Box<Node>>;

    // Rules
    fn rule_starts_now(parser: &mut Parser) -> bool;
    fn starts_now(parser: &mut Parser) -> bool {
        Self::prefix_operator_power(parser.current_token()).is_some()
            || Self::rule_starts_now(parser)
    }

    fn parse0(parser: &mut Parser) -> ParseResult<Box<Node>>;

    fn parse(parser: &mut Parser) -> ParseResult<Box<Node>> {
        Self::parse_bp(parser, 0)
    }

    fn parse_lhs(parser: &mut Parser) -> ParseResult<Box<Node>> {
        if parser.current_token().is(TokenKind::tLPAREN) {
            let begin_t = parser.take_token();
            let lhs = Self::parse_bp(parser, 0).unwrap();
            let end_t = parser.expect_token(TokenKind::tRPAREN).unwrap();
            Ok(Builder::begin(begin_t, vec![*lhs], end_t))
        } else if let Some((_, r_bp)) = Self::prefix_operator_power(parser.current_token()) {
            let op_t = parser.take_token();
            let rhs = Self::parse_bp(parser, r_bp).unwrap();
            Self::build_prefix_op(op_t, rhs, parser)
        } else {
            Self::parse0(parser)
        }
    }

    fn parse_with_lhs(
        parser: &mut Parser,
        mut lhs: Box<Node>,
        min_bp: u8,
    ) -> ParseResult<Box<Node>> {
        loop {
            let op_t = parser.current_token();

            if op_t.is(TokenKind::tEOF) {
                break;
            }

            if let Some((l_bp, _)) = Self::postfix_operator_power(op_t) {
                if l_bp < min_bp {
                    break;
                }
                parser.skip_token();

                lhs = Self::build_postfix_op(op_t, lhs, parser).unwrap();
                continue;
            }

            if let Some((l_bp, r_bp)) = Self::binary_operator_power(op_t) {
                if l_bp < min_bp {
                    break;
                }
                parser.skip_token();

                lhs = Self::build_binary_op(op_t, lhs, parser, r_bp).unwrap();
                continue;
            }

            break;
        }

        Ok(lhs)
    }

    fn parse_bp(parser: &mut Parser, min_bp: u8) -> ParseResult<Box<Node>> {
        let lhs = Self::parse_lhs(parser).unwrap();

        Self::parse_with_lhs(parser, lhs, min_bp)
    }
}

struct ExprType;
impl ValueType for ExprType {
    // Prefix operators
    fn prefix_operator_power(token: Token) -> Option<(u8, u8)> {
        match token.kind {
            TokenKind::tBANG | TokenKind::kNOT => token.kind.precedence(),
            _ => ArgType::prefix_operator_power(token),
        }
    }
    fn build_prefix_op(op_t: Token, arg: Box<Node>, parser: &mut Parser) -> ParseResult<Box<Node>> {
        todo!()
    }

    // Binary operators
    fn binary_operator_power(token: Token) -> Option<(u8, u8)> {
        match token.kind {
            TokenKind::kAND | TokenKind::kOR | TokenKind::tASSOC | TokenKind::kIN => {
                token.kind.precedence()
            }
            _ => ArgType::binary_operator_power(token),
        }
    }
    fn build_binary_op(
        op_t: Token,
        lhs: Box<Node>,
        parser: &mut Parser,
        r_bp: u8,
    ) -> ParseResult<Box<Node>> {
        todo!()
    }

    // Postfix operators
    fn postfix_operator_power(token: Token) -> Option<(u8, u8)> {
        ArgType::postfix_operator_power(token)
    }
    fn build_postfix_op(
        op_t: Token,
        arg: Box<Node>,
        parser: &mut Parser,
    ) -> ParseResult<Box<Node>> {
        todo!()
    }

    // Rules
    fn rule_starts_now(parser: &mut Parser) -> bool {
        Primary::starts_now(parser) || EndlessMethodDef::<Value<ArgType>>::starts_now(parser)
    }
    fn parse0(parser: &mut Parser) -> ParseResult<Box<Node>> {
        todo!()
    }
}

struct StmtType;
impl ValueType for StmtType {
    // Prefix operators
    fn prefix_operator_power(token: Token) -> Option<(u8, u8)> {
        ExprType::prefix_operator_power(token)
    }
    fn build_prefix_op(op_t: Token, arg: Box<Node>, parser: &mut Parser) -> ParseResult<Box<Node>> {
        todo!()
    }

    // Binary operators
    fn binary_operator_power(token: Token) -> Option<(u8, u8)> {
        match token.kind {
            TokenKind::tEQL | TokenKind::kIF | TokenKind::kWHILE | TokenKind::kRESCUE => {
                token.kind.precedence()
            }
            _ => ExprType::binary_operator_power(token),
        }
    }
    fn build_binary_op(
        op_t: Token,
        lhs: Box<Node>,
        parser: &mut Parser,
        r_bp: u8,
    ) -> ParseResult<Box<Node>> {
        todo!()
    }

    // Postfix operators
    fn postfix_operator_power(token: Token) -> Option<(u8, u8)> {
        ExprType::postfix_operator_power(token)
    }
    fn build_postfix_op(
        op_t: Token,
        arg: Box<Node>,
        parser: &mut Parser,
    ) -> ParseResult<Box<Node>> {
        todo!()
    }

    // Rules
    fn rule_starts_now(parser: &mut Parser) -> bool {
        Primary::starts_now(parser) || EndlessMethodDef::<Value<ArgType>>::starts_now(parser)
    }
    fn parse0(parser: &mut Parser) -> ParseResult<Box<Node>> {
        todo!()
    }
}