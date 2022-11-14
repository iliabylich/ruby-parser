use crate::{
    parser::{Captured, ParseError, ParseResult, Rule},
    Parser, Token, TokenKind,
};

trait TokenBasedRule<const N: usize = 0> {
    const TOKENS: [TokenKind; N];
}

const fn concat<const A: usize, const B: usize, const C: usize>(
    lhs: [TokenKind; A],
    rhs: [TokenKind; B],
) -> [TokenKind; C] {
    assert!(A + B == C);

    let mut result = [TokenKind::tEOF; C];

    let mut i = 0;
    while i < A {
        result[i] = lhs[i];
        i += 1;
    }

    while i < A + B {
        result[i] = rhs[i - A];
        i += 1;
    }

    let result = sort_arr(result);

    result
}

const fn sort_arr<const N: usize>(mut arr: [TokenKind; N]) -> [TokenKind; N] {
    loop {
        let mut swapped = false;
        let mut i = 1;
        while i < arr.len() {
            if arr[i - 1] as u8 > arr[i] as u8 {
                let left = arr[i - 1];
                let right = arr[i];
                arr[i - 1] = right;
                arr[i] = left;
                swapped = true;
            }
            i += 1;
        }
        if !swapped {
            break;
        }
    }
    arr
}

macro_rules! concat_array_const {
    ($($arr:expr),*) => {
        concat_array_const!(@concat
            $( [$arr ; $arr.len()] )*
        )
    };

    (@concat [$a:expr; $a_len:expr]) => {
        $a
    };

    (@concat [$a:expr; $a_len:expr] [$b:expr; $b_len:expr] $($tail:tt)*) => {
        concat_array_const!(
            @concat
            [concat::<{ $a_len }, { $b_len }, { $a_len + $b_len }>($a, $b); $a_len + $b_len]
            $($tail)*
        )
    };
}

impl<const N: usize, T> Rule<N> for T
where
    T: TokenBasedRule<N>,
{
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is_one_of_sorted(Self::TOKENS)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if Self::starts_now(parser) {
            let token = parser.current_token();
            parser.skip_token();
            Ok(token)
        } else {
            Err(ParseError {
                error: (),
                captured: Captured::default(),
            })
        }
    }
}

pub(crate) struct OperationT;
impl TokenBasedRule<3> for OperationT {
    const TOKENS: [TokenKind; 3] = concat(IdOrConstT::TOKENS, [TokenKind::tFID]);
}

pub(crate) struct Operation2T;
impl TokenBasedRule<1> for Operation2T {
    const TOKENS: [TokenKind; 1] = concat(OperationT::TOKENS, OpT::TOKENS);
}

pub(crate) struct Operation3T;
impl TokenBasedRule<1> for Operation3T {
    const TOKENS: [TokenKind; 1] = concat(OpT::TOKENS, [TokenKind::tIDENTIFIER, TokenKind::tFID]);
}

pub(crate) struct FnameT;
impl TokenBasedRule<73> for FnameT {
    const TOKENS: [TokenKind; 73] = concat_array_const!(
        ReswordsT::TOKENS,
        IdOrConstT::TOKENS,
        OpT::TOKENS,
        [TokenKind::tFID]
    );
}

pub(crate) struct SimpleNumericT;
impl TokenBasedRule<4> for SimpleNumericT {
    const TOKENS: [TokenKind; 4] = [
        TokenKind::tINTEGER,
        TokenKind::tFLOAT,
        TokenKind::tRATIONAL,
        TokenKind::tIMAGINARY,
    ];
}

pub(crate) struct UserVariableT;
impl TokenBasedRule<5> for UserVariableT {
    const TOKENS: [TokenKind; 5] = concat(IdOrConstT::TOKENS, NonLocalVarT::TOKENS);
}

pub(crate) struct KeywordVariableT;
impl TokenBasedRule<7> for KeywordVariableT {
    const TOKENS: [TokenKind; 7] = [
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
impl TokenBasedRule<1> for VarRefT {
    const TOKENS: [TokenKind; 1] = concat(UserVariableT::TOKENS, KeywordVariableT::TOKENS);
}

pub(crate) struct BackRefT;
impl TokenBasedRule<2> for BackRefT {
    const TOKENS: [TokenKind; 2] = [TokenKind::tNTH_REF, TokenKind::tBACK_REF];
}

pub(crate) struct CnameT;
impl TokenBasedRule<2> for CnameT {
    const TOKENS: [TokenKind; 2] = IdOrConstT::TOKENS;
}

pub(crate) struct StringDvarT;
impl TokenBasedRule<1> for StringDvarT {
    const TOKENS: [TokenKind; 1] = concat(FnameT::TOKENS, NonLocalVarT::TOKENS);
}

pub(crate) struct SymT;
impl TokenBasedRule<48> for SymT {
    const TOKENS: [TokenKind; 48] = concat(FnameT::TOKENS, NonLocalVarT::TOKENS);
}

pub(crate) struct CallOpT;
impl TokenBasedRule<2> for CallOpT {
    const TOKENS: [TokenKind; 2] = [TokenKind::tDOT, TokenKind::tANDDOT];
}

pub(crate) struct CallOp2T;
impl TokenBasedRule<1> for CallOp2T {
    const TOKENS: [TokenKind; 1] = concat(CallOpT::TOKENS, [TokenKind::tCOLON2]);
}

pub(crate) struct MethodNameT;
impl TokenBasedRule<2> for MethodNameT {
    const TOKENS: [TokenKind; 2] = IdOrConstT::TOKENS;
}

pub(crate) struct DoT;
impl TokenBasedRule<3> for DoT {
    const TOKENS: [TokenKind; 3] = concat(TermT::TOKENS, [TokenKind::kDO]);
}

pub(crate) struct TermT;
impl TokenBasedRule<2> for TermT {
    const TOKENS: [TokenKind; 2] = [TokenKind::tSEMI, TokenKind::tNL];
}

struct OpT;
impl TokenBasedRule<29> for OpT {
    const TOKENS: [TokenKind; 29] = [
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
impl TokenBasedRule<41> for ReswordsT {
    const TOKENS: [TokenKind; 41] = [
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
impl TokenBasedRule<3> for NonLocalVarT {
    const TOKENS: [TokenKind; 3] = [TokenKind::tIVAR, TokenKind::tGVAR, TokenKind::tCVAR];
}

struct IdOrConstT;
impl TokenBasedRule<2> for IdOrConstT {
    const TOKENS: [TokenKind; 2] = [TokenKind::tIDENTIFIER, TokenKind::tCONSTANT];
}

#[test]
fn test_id_or_const_t() {
    use crate::loc::loc;
    use crate::token::token;

    let mut parser = Parser::new(b"foo");
    assert!(IdOrConstT::starts_now(&mut parser));
    assert_eq!(
        IdOrConstT::parse(&mut parser),
        Ok(token!(tIDENTIFIER, loc!(0, 3)))
    );

    let mut parser = Parser::new(b"42");
    assert!(!IdOrConstT::starts_now(&mut parser));
}
