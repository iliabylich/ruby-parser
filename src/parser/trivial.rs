use crate::parser::base::{Captured, ParseError, ParseResult, Rule};
use crate::{Parser, Token, TokenKind};

trait TokenBasedRule {
    fn _starts_now(parser: &mut Parser) -> bool;
}

impl<T> Rule for T
where
    T: TokenBasedRule,
{
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        Self::_starts_now(parser)
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
    fn _starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tFID) || IdOrConstT::starts_now(parser)
    }
}

pub(crate) struct Operation2T;
impl TokenBasedRule for Operation2T {
    fn _starts_now(parser: &mut Parser) -> bool {
        OperationT::starts_now(parser) || OpT::starts_now(parser)
    }
}

pub(crate) struct Operation3T;
impl TokenBasedRule for Operation3T {
    fn _starts_now(parser: &mut Parser) -> bool {
        parser
            .current_token()
            .is_one_of([TokenKind::tIDENTIFIER, TokenKind::tFID])
            || OpT::starts_now(parser)
    }
}

pub(crate) struct FnameT;
impl TokenBasedRule for FnameT {
    fn _starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tFID)
            || IdOrConstT::starts_now(parser)
            || OpT::starts_now(parser)
            || ReswordsT::starts_now(parser)
    }
}

pub(crate) struct SimpleNumericT;
impl TokenBasedRule for SimpleNumericT {
    fn _starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is_one_of([
            TokenKind::tINTEGER,
            TokenKind::tFLOAT,
            TokenKind::tRATIONAL,
            TokenKind::tIMAGINARY,
        ])
    }
}

pub(crate) struct UserVariableT;
impl TokenBasedRule for UserVariableT {
    fn _starts_now(parser: &mut Parser) -> bool {
        IdOrConstT::starts_now(parser) || NonLocalVarT::starts_now(parser)
    }
}

pub(crate) struct KeywordVariableT;
impl TokenBasedRule for KeywordVariableT {
    fn _starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is_one_of([
            TokenKind::kNIL,
            TokenKind::kSELF,
            TokenKind::kTRUE,
            TokenKind::kFALSE,
            TokenKind::k__FILE__,
            TokenKind::k__LINE__,
            TokenKind::k__ENCODING__,
        ])
    }
}

pub(crate) struct VarRefT;
impl TokenBasedRule for VarRefT {
    fn _starts_now(parser: &mut Parser) -> bool {
        UserVariableT::starts_now(parser) || KeywordVariableT::starts_now(parser)
    }
}

pub(crate) struct BackRefT;
impl TokenBasedRule for BackRefT {
    fn _starts_now(parser: &mut Parser) -> bool {
        parser
            .current_token()
            .is_one_of([TokenKind::tNTH_REF, TokenKind::tBACK_REF])
    }
}

pub(crate) struct CnameT;
impl TokenBasedRule for CnameT {
    fn _starts_now(parser: &mut Parser) -> bool {
        IdOrConstT::starts_now(parser)
    }
}

pub(crate) struct StringDvarT;
impl TokenBasedRule for StringDvarT {
    fn _starts_now(parser: &mut Parser) -> bool {
        FnameT::starts_now(parser) || NonLocalVarT::starts_now(parser)
    }
}

pub(crate) struct SymT;
impl TokenBasedRule for SymT {
    fn _starts_now(parser: &mut Parser) -> bool {
        FnameT::starts_now(parser) || NonLocalVarT::starts_now(parser)
    }
}

pub(crate) struct CallOpT;
impl TokenBasedRule for CallOpT {
    fn _starts_now(parser: &mut Parser) -> bool {
        parser
            .current_token()
            .is_one_of([TokenKind::tDOT, TokenKind::tANDDOT])
    }
}

pub(crate) struct CallOp2T;
impl TokenBasedRule for CallOp2T {
    fn _starts_now(parser: &mut Parser) -> bool {
        CallOpT::starts_now(parser) || parser.current_token().is(TokenKind::tCOLON2)
    }
}

pub(crate) struct MethodNameT;
impl TokenBasedRule for MethodNameT {
    fn _starts_now(parser: &mut Parser) -> bool {
        IdOrConstT::starts_now(parser)
    }
}

pub(crate) struct DoT;
impl TokenBasedRule for DoT {
    fn _starts_now(parser: &mut Parser) -> bool {
        TermT::starts_now(parser) || parser.current_token().is(TokenKind::kDO)
    }
}

pub(crate) struct TermT;
impl TokenBasedRule for TermT {
    fn _starts_now(parser: &mut Parser) -> bool {
        parser
            .current_token()
            .is_one_of([TokenKind::tSEMI, TokenKind::tNL])
    }
}

struct OpT;
impl TokenBasedRule for OpT {
    fn _starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is_one_of([
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
        ])
    }
}

struct ReswordsT;
impl TokenBasedRule for ReswordsT {
    fn _starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is_one_of([
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
        ])
    }
}

struct NonLocalVarT;
impl TokenBasedRule for NonLocalVarT {
    fn _starts_now(parser: &mut Parser) -> bool {
        parser
            .current_token()
            .is_one_of([TokenKind::tIVAR, TokenKind::tGVAR, TokenKind::tCVAR])
    }
}

struct IdOrConstT;
impl TokenBasedRule for IdOrConstT {
    fn _starts_now(parser: &mut Parser) -> bool {
        parser
            .current_token()
            .is_one_of([TokenKind::tIDENTIFIER, TokenKind::tCONSTANT])
    }
}
