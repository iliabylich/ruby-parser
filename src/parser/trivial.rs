use crate::{
    builder::Builder,
    parser::{Captured, ParseError, ParseResult, Rule},
    Node, Parser, Token, TokenKind,
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
            Ok(parser.take_token())
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
impl TokenBasedRule<32> for Operation2T {
    const TOKENS: [TokenKind; 32] = concat(OperationT::TOKENS, OpT::TOKENS);
}

pub(crate) struct Operation3T;
impl TokenBasedRule<31> for Operation3T {
    const TOKENS: [TokenKind; 31] = concat(OpT::TOKENS, [TokenKind::tIDENTIFIER, TokenKind::tFID]);
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

pub(crate) struct SimpleNumeric;
impl Rule for SimpleNumeric {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is_one_of([
            TokenKind::tINTEGER,
            TokenKind::tFLOAT,
            TokenKind::tRATIONAL,
            TokenKind::tIMAGINARY,
        ])
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let numeric_t = parser.take_token();
        let node = match numeric_t.kind {
            TokenKind::tINTEGER => Builder::integer(numeric_t, parser.buffer()),
            TokenKind::tFLOAT => Builder::float(numeric_t, parser.buffer()),
            TokenKind::tRATIONAL => Builder::rational(numeric_t, parser.buffer()),
            TokenKind::tIMAGINARY => Builder::complex(numeric_t, parser.buffer()),
            _ => unreachable!(),
        };
        Ok(node)
    }
}

struct UserVariable;
impl Rule for UserVariable {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        IdOrConstT::starts_now(parser) || NonLocalVar::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if IdOrConstT::starts_now(parser) {
            let token = IdOrConstT::parse(parser).unwrap();
            Ok(Builder::lvar(token, parser.buffer()))
        } else if NonLocalVar::starts_now(parser) {
            NonLocalVar::parse(parser)
        } else {
            unreachable!()
        }
    }
}

pub(crate) struct KeywordVariable;
impl Rule for KeywordVariable {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
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

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let token = parser.take_token();
        let node = match token.kind {
            TokenKind::kNIL => Builder::nil(token),
            TokenKind::kSELF => Builder::self_(token),
            TokenKind::kTRUE => Builder::true_(token),
            TokenKind::kFALSE => Builder::false_(token),
            TokenKind::k__FILE__ => Builder::__file__(token),
            TokenKind::k__LINE__ => Builder::__line__(token),
            TokenKind::k__ENCODING__ => Builder::__encoding__(token),
            _ => unreachable!(),
        };
        Ok(node)
    }
}

pub(crate) struct VarRef;
impl Rule for VarRef {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        UserVariable::starts_now(parser) || KeywordVariable::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if UserVariable::starts_now(parser) {
            UserVariable::parse(parser)
        } else if KeywordVariable::starts_now(parser) {
            KeywordVariable::parse(parser)
        } else {
            unreachable!()
        }
    }
}

pub(crate) struct BackRef;
impl Rule for BackRef {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser
            .current_token()
            .is_one_of([TokenKind::tNTH_REF, TokenKind::tBACK_REF])
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let token = parser.take_token();
        let node = match token.kind {
            TokenKind::tNTH_REF => Builder::nth_ref(token, parser.buffer()),
            TokenKind::tBACK_REF => Builder::back_ref(token, parser.buffer()),
            _ => unreachable!(),
        };
        Ok(node)
    }
}

pub(crate) struct CnameT;
impl TokenBasedRule<2> for CnameT {
    const TOKENS: [TokenKind; 2] = IdOrConstT::TOKENS;
}

pub(crate) struct StringDvar;
impl Rule for StringDvar {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        NonLocalVar::starts_now(parser) || BackRef::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if NonLocalVar::starts_now(parser) {
            NonLocalVar::parse(parser)
        } else if BackRef::starts_now(parser) {
            BackRef::parse(parser)
        } else {
            unreachable!()
        }
    }
}

pub(crate) struct SymT;
impl Rule for SymT {
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        FnameT::starts_now(parser)
            || parser.current_token().is_one_of([
                TokenKind::tIVAR,
                TokenKind::tCVAR,
                TokenKind::tGVAR,
            ])
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if Self::starts_now(parser) {
            Ok(parser.take_token())
        } else {
            unreachable!()
        }
    }
}

pub(crate) struct CallOpT;
impl TokenBasedRule<2> for CallOpT {
    const TOKENS: [TokenKind; 2] = [TokenKind::tDOT, TokenKind::tANDDOT];
}

pub(crate) struct CallOp2T;
impl TokenBasedRule<1> for CallOp2T {
    const TOKENS: [TokenKind; 1] = concat(CallOpT::TOKENS, [TokenKind::tCOLON2]);
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

pub(crate) struct Ivar;
impl Rule for Ivar {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tIVAR)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let ivar_t = parser.take_token();
        Ok(Builder::ivar(ivar_t, parser.buffer()))
    }
}

pub(crate) struct Cvar;
impl Rule for Cvar {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tCVAR)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let cvar_t = parser.take_token();
        Ok(Builder::cvar(cvar_t, parser.buffer()))
    }
}

pub(crate) struct Gvar;
impl Rule for Gvar {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tGVAR)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let gvar_t = parser.take_token();
        Ok(Builder::gvar(gvar_t, parser.buffer()))
    }
}

struct NonLocalVar;
impl Rule for NonLocalVar {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        Ivar::starts_now(parser) || Cvar::starts_now(parser) || Gvar::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if Ivar::starts_now(parser) {
            Ivar::parse(parser)
        } else if Cvar::starts_now(parser) {
            Cvar::parse(parser)
        } else if Gvar::starts_now(parser) {
            Gvar::parse(parser)
        } else {
            unreachable!()
        }
    }
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
