use crate::{
    parser::{
        base::{ParseResult, Rule},
        EndlessMethodDef, Primary,
    },
    Node, Parser, TokenKind,
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
    const PREFIX: &'static [TokenKind];
    const POSTFIX: &'static [TokenKind];
    const BINARY: &'static [TokenKind];

    fn starts_now(parser: &mut Parser) -> bool {
        Self::prefix_operator_starts_now(parser) || Self::rule_starts_now(parser)
    }

    fn own_prefix_operator_starts_now(parser: &mut Parser) -> bool {
        Self::PREFIX
            .iter()
            .any(|token_kind| parser.current_token().is(*token_kind))
    }
    fn prefix_operator_starts_now(parser: &mut Parser) -> bool;

    fn postfix_operator_starts_now(parser: &mut Parser) -> bool;
    fn own_postfix_operator_starts_now(parser: &mut Parser) -> bool {
        Self::POSTFIX
            .iter()
            .any(|token_kind| parser.current_token().is(*token_kind))
    }

    fn binary_operator_starts_now(parser: &mut Parser) -> bool;
    fn own_binary_operator_starts_now(parser: &mut Parser) -> bool {
        Self::BINARY
            .iter()
            .any(|token_kind| parser.current_token().is(*token_kind))
    }

    fn rule_starts_now(parser: &mut Parser) -> bool;

    fn parse(parser: &mut Parser) -> ParseResult<Box<Node>> {
        todo!()
    }
}

struct ArgType;
impl ValueType for ArgType {
    const PREFIX: &'static [TokenKind] = &[
        TokenKind::tDOT2,
        TokenKind::tDOT3,
        TokenKind::tPLUS,
        TokenKind::tMINUS,
        TokenKind::tBANG,
        TokenKind::tTILDE,
        TokenKind::kDEFINED,
    ];

    const BINARY: &'static [TokenKind] = &[
        // Assignments that are parsed as "binary operators"
        TokenKind::tEQL,
        TokenKind::tOP_ASGN,
        // Standard binary operators
        TokenKind::tDOT2,
        TokenKind::tDOT3,
        TokenKind::tPLUS,
        TokenKind::tMINUS,
        TokenKind::tSTAR,
        TokenKind::tDIVIDE,
        TokenKind::tPERCENT,
        TokenKind::tDSTAR,
        TokenKind::tPIPE,
        TokenKind::tCARET,
        TokenKind::tAMPER,
        TokenKind::tCMP,
        TokenKind::tEQ,
        TokenKind::tEQQ,
        TokenKind::tNEQ,
        TokenKind::tMATCH,
        TokenKind::tNMATCH,
        TokenKind::tLSHFT,
        TokenKind::tANDOP,
        TokenKind::tOROP,
        TokenKind::tGT,
        TokenKind::tLT,
        TokenKind::tGEQ,
        TokenKind::tLEQ,
        // Ternary operator that is also a "binary operator"
        TokenKind::tEH,
        // 'rescue' keyword, also a "binary"
        TokenKind::kRESCUE,
    ];

    const POSTFIX: &'static [TokenKind] = &[TokenKind::tDOT2, TokenKind::tDOT3];

    fn rule_starts_now(parser: &mut Parser) -> bool {
        Primary::starts_now(parser) || EndlessMethodDef::<Value<ArgType>>::starts_now(parser)
    }

    fn prefix_operator_starts_now(parser: &mut Parser) -> bool {
        Self::own_prefix_operator_starts_now(parser)
    }

    fn postfix_operator_starts_now(parser: &mut Parser) -> bool {
        Self::own_postfix_operator_starts_now(parser)
    }

    fn binary_operator_starts_now(parser: &mut Parser) -> bool {
        Self::own_binary_operator_starts_now(parser)
    }
}

struct ExprType;
impl ValueType for ExprType {
    const PREFIX: &'static [TokenKind] = &[TokenKind::tBANG, TokenKind::kNOT];

    const POSTFIX: &'static [TokenKind] = &[];

    const BINARY: &'static [TokenKind] = &[
        TokenKind::kAND,
        TokenKind::kOR,
        TokenKind::tASSOC,
        TokenKind::kIN,
    ];

    fn rule_starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn prefix_operator_starts_now(parser: &mut Parser) -> bool {
        Self::own_prefix_operator_starts_now(parser) || ArgType::prefix_operator_starts_now(parser)
    }

    fn postfix_operator_starts_now(parser: &mut Parser) -> bool {
        Self::own_postfix_operator_starts_now(parser)
            || ArgType::postfix_operator_starts_now(parser)
    }

    fn binary_operator_starts_now(parser: &mut Parser) -> bool {
        Self::own_binary_operator_starts_now(parser) || ArgType::binary_operator_starts_now(parser)
    }
}

struct StmtType;
impl ValueType for StmtType {
    const PREFIX: &'static [TokenKind] = &[];

    const POSTFIX: &'static [TokenKind] = &[];

    const BINARY: &'static [TokenKind] = &[
        TokenKind::tEQL,
        TokenKind::kIF,
        TokenKind::kWHILE,
        TokenKind::kRESCUE,
    ];

    fn rule_starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn prefix_operator_starts_now(parser: &mut Parser) -> bool {
        Self::own_prefix_operator_starts_now(parser) || ExprType::prefix_operator_starts_now(parser)
    }

    fn postfix_operator_starts_now(parser: &mut Parser) -> bool {
        Self::own_postfix_operator_starts_now(parser)
            || ExprType::postfix_operator_starts_now(parser)
    }

    fn binary_operator_starts_now(parser: &mut Parser) -> bool {
        Self::own_binary_operator_starts_now(parser) || ExprType::binary_operator_starts_now(parser)
    }
}
