use crate::loc::loc;
use crate::parser::base::{Captured, ParseError, ParseResult, Rule};
use crate::token::token;
use crate::{Parser, Token, TokenKind};

trait TokenBasedRule {
    const TOKENS: &'static [TokenKind];
}

const fn concat<const N: usize>(
    lhs: &'static [TokenKind],
    rhs: &'static [TokenKind],
) -> [TokenKind; N] {
    assert!(lhs.len() + rhs.len() == N);

    let mut result = [TokenKind::tEOF; N];

    let mut i = 0;
    while i < lhs.len() {
        result[i] = lhs[i];
        i += 1;
    }

    while i < lhs.len() + rhs.len() {
        result[i] = rhs[i - lhs.len()];
        i += 1;
    }

    result
}

impl<T> Rule for T
where
    T: TokenBasedRule,
{
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is_one_of(Self::TOKENS)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if Self::starts_now(parser) {
            Ok(parser.current_token())
        } else {
            Err(ParseError {
                error: (),
                captured: Captured::default(),
            })
        }
    }
}

pub(crate) struct OperationT;
impl TokenBasedRule for OperationT {
    const TOKENS: &'static [TokenKind] =
        &concat::<{ IdOrConstT::TOKENS.len() + 1 }>(IdOrConstT::TOKENS, &[TokenKind::tFID]);
}

pub(crate) struct Operation2T;
impl TokenBasedRule for Operation2T {
    const TOKENS: &'static [TokenKind] = &concat::<{ OperationT::TOKENS.len() + OpT::TOKENS.len() }>(
        OperationT::TOKENS,
        OpT::TOKENS,
    );
}

pub(crate) struct Operation3T;
impl TokenBasedRule for Operation3T {
    const TOKENS: &'static [TokenKind] = &concat::<{ OpT::TOKENS.len() + 2 }>(
        OpT::TOKENS,
        &[TokenKind::tIDENTIFIER, TokenKind::tFID],
    );
}

pub(crate) struct FnameT;
impl TokenBasedRule for FnameT {
    const TOKENS: &'static [TokenKind] =
        &concat::<{ ReswordsT::TOKENS.len() + IdOrConstT::TOKENS.len() + OpT::TOKENS.len() + 1 }>(
            &concat::<{ ReswordsT::TOKENS.len() + IdOrConstT::TOKENS.len() + OpT::TOKENS.len() }>(
                &concat::<{ ReswordsT::TOKENS.len() + IdOrConstT::TOKENS.len() }>(
                    ReswordsT::TOKENS,
                    IdOrConstT::TOKENS,
                ),
                OpT::TOKENS,
            ),
            &[TokenKind::tFID],
        );
}

pub(crate) struct SimpleNumericT;
impl TokenBasedRule for SimpleNumericT {
    const TOKENS: &'static [TokenKind] = &[
        TokenKind::tINTEGER,
        TokenKind::tFLOAT,
        TokenKind::tRATIONAL,
        TokenKind::tIMAGINARY,
    ];
}

pub(crate) struct UserVariableT;
impl TokenBasedRule for UserVariableT {
    const TOKENS: &'static [TokenKind] = &concat::<
        { IdOrConstT::TOKENS.len() + NonLocalVarT::TOKENS.len() },
    >(IdOrConstT::TOKENS, NonLocalVarT::TOKENS);
}

pub(crate) struct KeywordVariableT;
impl TokenBasedRule for KeywordVariableT {
    const TOKENS: &'static [TokenKind] = &[
        TokenKind::kNIL,
        TokenKind::kSELF,
        TokenKind::kTRUE,
        TokenKind::kFALSE,
        TokenKind::k__FILE__,
        TokenKind::k__LINE__,
        TokenKind::k__ENCODING__,
    ];
}

pub(crate) struct VarRefT;
impl TokenBasedRule for VarRefT {
    const TOKENS: &'static [TokenKind] = &concat::<
        { UserVariableT::TOKENS.len() + KeywordVariableT::TOKENS.len() },
    >(UserVariableT::TOKENS, KeywordVariableT::TOKENS);
}

pub(crate) struct BackRefT;
impl TokenBasedRule for BackRefT {
    const TOKENS: &'static [TokenKind] = &[TokenKind::tNTH_REF, TokenKind::tBACK_REF];
}

pub(crate) struct CnameT;
impl TokenBasedRule for CnameT {
    const TOKENS: &'static [TokenKind] = IdOrConstT::TOKENS;
}

pub(crate) struct StringDvarT;
impl TokenBasedRule for StringDvarT {
    const TOKENS: &'static [TokenKind] = &concat::<
        { FnameT::TOKENS.len() + NonLocalVarT::TOKENS.len() },
    >(FnameT::TOKENS, NonLocalVarT::TOKENS);
}

pub(crate) struct SymT;
impl TokenBasedRule for SymT {
    const TOKENS: &'static [TokenKind] = &concat::<
        { FnameT::TOKENS.len() + NonLocalVarT::TOKENS.len() },
    >(FnameT::TOKENS, NonLocalVarT::TOKENS);
}

pub(crate) struct CallOpT;
impl TokenBasedRule for CallOpT {
    const TOKENS: &'static [TokenKind] = &[TokenKind::tDOT, TokenKind::tANDDOT];
}

pub(crate) struct CallOp2T;
impl TokenBasedRule for CallOp2T {
    const TOKENS: &'static [TokenKind] =
        &concat::<{ CallOpT::TOKENS.len() + 1 }>(CallOpT::TOKENS, &[TokenKind::tCOLON2]);
}

pub(crate) struct MethodNameT;
impl TokenBasedRule for MethodNameT {
    const TOKENS: &'static [TokenKind] = IdOrConstT::TOKENS;
}

pub(crate) struct DoT;
impl TokenBasedRule for DoT {
    const TOKENS: &'static [TokenKind] =
        &concat::<{ TermT::TOKENS.len() + 1 }>(TermT::TOKENS, &[TokenKind::kDO]);
}

pub(crate) struct TermT;
impl TokenBasedRule for TermT {
    const TOKENS: &'static [TokenKind] = &[TokenKind::tSEMI, TokenKind::tNL];
}

struct OpT;
impl TokenBasedRule for OpT {
    const TOKENS: &'static [TokenKind] = &[
        TokenKind::tPIPE,
        TokenKind::tCARET,
        TokenKind::tAMPER,
        TokenKind::tCMP,
        TokenKind::tEQ,
        TokenKind::tEQQ,
        TokenKind::tMATCH,
        TokenKind::tNMATCH,
        TokenKind::tGT,
        TokenKind::tGEQ,
        TokenKind::tLT,
        TokenKind::tLEQ,
        TokenKind::tNEQ,
        TokenKind::tLSHFT,
        TokenKind::tRSHFT,
        TokenKind::tPLUS,
        TokenKind::tMINUS,
        TokenKind::tSTAR,
        TokenKind::tSTAR,
        TokenKind::tDIVIDE,
        TokenKind::tPERCENT,
        TokenKind::tDSTAR,
        TokenKind::tBANG,
        TokenKind::tTILDE,
        TokenKind::tUPLUS,
        TokenKind::tUMINUS,
        TokenKind::tAREF,
        TokenKind::tASET,
        TokenKind::tBACK_REF,
    ];
}

struct ReswordsT;
impl TokenBasedRule for ReswordsT {
    const TOKENS: &'static [TokenKind] = &[
        TokenKind::k__LINE__,
        TokenKind::k__FILE__,
        TokenKind::k__ENCODING__,
        TokenKind::klBEGIN,
        TokenKind::klEND,
        TokenKind::kALIAS,
        TokenKind::kAND,
        TokenKind::kBEGIN,
        TokenKind::kBREAK,
        TokenKind::kCASE,
        TokenKind::kCLASS,
        TokenKind::kDEF,
        TokenKind::kDEFINED,
        TokenKind::kDO,
        TokenKind::kELSE,
        TokenKind::kELSIF,
        TokenKind::kEND,
        TokenKind::kENSURE,
        TokenKind::kFALSE,
        TokenKind::kFOR,
        TokenKind::kIN,
        TokenKind::kMODULE,
        TokenKind::kNEXT,
        TokenKind::kNIL,
        TokenKind::kNOT,
        TokenKind::kOR,
        TokenKind::kREDO,
        TokenKind::kRESCUE,
        TokenKind::kRETRY,
        TokenKind::kRETURN,
        TokenKind::kSELF,
        TokenKind::kSUPER,
        TokenKind::kTHEN,
        TokenKind::kTRUE,
        TokenKind::kUNDEF,
        TokenKind::kWHEN,
        TokenKind::kYIELD,
        TokenKind::kIF,
        TokenKind::kUNLESS,
        TokenKind::kWHILE,
        TokenKind::kUNTIL,
    ];
}

struct NonLocalVarT;
impl TokenBasedRule for NonLocalVarT {
    const TOKENS: &'static [TokenKind] = &[TokenKind::tIVAR, TokenKind::tGVAR, TokenKind::tCVAR];
}

struct IdOrConstT;
impl TokenBasedRule for IdOrConstT {
    const TOKENS: &'static [TokenKind] = &[TokenKind::tIDENTIFIER, TokenKind::tCONSTANT];
}

#[test]
fn test_id_or_const_t() {
    let mut parser = Parser::new(b"foo");
    assert!(IdOrConstT::starts_now(&mut parser));
    assert_eq!(
        IdOrConstT::parse(&mut parser),
        Ok(token!(tIDENTIFIER, loc!(0, 3)))
    );

    let mut parser = Parser::new(b"42");
    assert!(!IdOrConstT::starts_now(&mut parser));
}
