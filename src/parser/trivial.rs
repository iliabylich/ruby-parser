use crate::{
    builder::Builder,
    parser::base::{at_most_one_is_true, Rule},
    Node, Parser, Token, TokenKind,
};

pub(crate) struct BackRef;
impl Rule for BackRef {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser
            .current_token()
            .is_one_of([TokenKind::tNTH_REF, TokenKind::tBACK_REF])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let token = parser.take_token();
        match token.kind {
            TokenKind::tNTH_REF => Builder::nth_ref(token, parser.buffer()),
            TokenKind::tBACK_REF => Builder::back_ref(token, parser.buffer()),
            _ => unreachable!(),
        }
    }
}

pub(crate) struct DotOrColon2T;
impl Rule for DotOrColon2T {
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        at_most_one_is_true([
            DotT::starts_now(parser),
            parser.current_token().is(TokenKind::tCOLON2),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if DotT::starts_now(parser) {
            DotT::parse(parser)
        } else if parser.current_token().is(TokenKind::tCOLON2) {
            parser.take_token()
        } else {
            unreachable!()
        }
    }
}

pub(crate) struct DotT;
impl Rule for DotT {
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        at_most_one_is_true([
            parser.current_token().is(TokenKind::tDOT),
            parser.current_token().is(TokenKind::tANDDOT),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if Self::starts_now(parser) {
            parser.take_token()
        } else {
            unreachable!()
        }
    }
}

pub(crate) struct CnameT;
impl Rule for CnameT {
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        IdOrConstT::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        IdOrConstT::parse(parser)
    }
}

pub(crate) struct DoT;
impl Rule for DoT {
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        at_most_one_is_true([
            TermT::starts_now(parser),
            parser.current_token().is(TokenKind::kDO),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if TermT::starts_now(parser) {
            TermT::parse(parser)
        } else if parser.current_token().is(TokenKind::kDO) {
            parser.take_token()
        } else {
            unreachable!()
        }
    }
}

pub(crate) struct FnameT;
impl Rule for FnameT {
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        at_most_one_is_true([
            ReswordsT::starts_now(parser),
            IdOrConstT::starts_now(parser),
            OpT::starts_now(parser),
            parser.current_token().is(TokenKind::tFID),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if ReswordsT::starts_now(parser) {
            ReswordsT::parse(parser)
        } else if IdOrConstT::starts_now(parser) {
            IdOrConstT::parse(parser)
        } else if OpT::starts_now(parser) {
            OpT::parse(parser)
        } else if parser.current_token().is(TokenKind::tFID) {
            parser.take_token()
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

    fn parse(parser: &mut Parser) -> Self::Output {
        let token = parser.take_token();
        match token.kind {
            TokenKind::kNIL => Builder::nil(token),
            TokenKind::kSELF => Builder::self_(token),
            TokenKind::kTRUE => Builder::true_(token),
            TokenKind::kFALSE => Builder::false_(token),
            TokenKind::k__FILE__ => Builder::__file__(token),
            TokenKind::k__LINE__ => Builder::__line__(token),
            TokenKind::k__ENCODING__ => Builder::__encoding__(token),
            _ => unreachable!(),
        }
    }
}

pub(crate) struct OperationT;
impl Rule for OperationT {
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        at_most_one_is_true([
            IdOrConstT::starts_now(parser),
            parser.current_token().is(TokenKind::tFID),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        parser.take_token()
    }
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

    fn parse(parser: &mut Parser) -> Self::Output {
        let numeric_t = parser.take_token();
        match numeric_t.kind {
            TokenKind::tINTEGER => Builder::integer(numeric_t, parser.buffer()),
            TokenKind::tFLOAT => Builder::float(numeric_t, parser.buffer()),
            TokenKind::tRATIONAL => Builder::rational(numeric_t, parser.buffer()),
            TokenKind::tIMAGINARY => Builder::complex(numeric_t, parser.buffer()),
            _ => unreachable!(),
        }
    }
}

pub(crate) struct StringDvar;
impl Rule for StringDvar {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        at_most_one_is_true([NonLocalVar::starts_now(parser), BackRef::starts_now(parser)])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
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
        at_most_one_is_true([
            FnameT::starts_now(parser),
            parser.current_token().is(TokenKind::tIVAR),
            parser.current_token().is(TokenKind::tCVAR),
            parser.current_token().is(TokenKind::tGVAR),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if Self::starts_now(parser) {
            parser.take_token()
        } else {
            unreachable!()
        }
    }
}

pub(crate) struct TermT;
impl Rule for TermT {
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        at_most_one_is_true([
            parser.current_token().is(TokenKind::tSEMI),
            parser.current_token().is(TokenKind::tNL),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if Self::starts_now(parser) {
            parser.take_token()
        } else {
            unreachable!()
        }
    }
}

pub(crate) struct VarRef;
impl Rule for VarRef {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        at_most_one_is_true([
            UserVariable::starts_now(parser),
            KeywordVariable::starts_now(parser),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if UserVariable::starts_now(parser) {
            UserVariable::parse(parser)
        } else if KeywordVariable::starts_now(parser) {
            KeywordVariable::parse(parser)
        } else {
            unreachable!()
        }
    }
}

pub(crate) struct OpT;
impl Rule for OpT {
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        let token = parser.current_token();

        at_most_one_is_true([
            token.is(TokenKind::tPIPE),
            token.is(TokenKind::tCARET),
            token.is(TokenKind::tAMPER),
            token.is(TokenKind::tCMP),
            token.is(TokenKind::tEQ),
            token.is(TokenKind::tEQQ),
            token.is(TokenKind::tMATCH),
            token.is(TokenKind::tNMATCH),
            token.is(TokenKind::tGT),
            token.is(TokenKind::tGEQ),
            token.is(TokenKind::tLT),
            token.is(TokenKind::tLEQ),
            token.is(TokenKind::tNEQ),
            token.is(TokenKind::tLSHFT),
            token.is(TokenKind::tRSHFT),
            token.is(TokenKind::tPLUS),
            token.is(TokenKind::tMINUS),
            token.is(TokenKind::tSTAR),
            token.is(TokenKind::tSTAR),
            token.is(TokenKind::tDIVIDE),
            token.is(TokenKind::tPERCENT),
            token.is(TokenKind::tDSTAR),
            token.is(TokenKind::tBANG),
            token.is(TokenKind::tTILDE),
            token.is(TokenKind::tUPLUS),
            token.is(TokenKind::tUMINUS),
            token.is(TokenKind::tAREF),
            token.is(TokenKind::tASET),
            token.is(TokenKind::tBACK_REF),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if Self::starts_now(parser) {
            parser.take_token()
        } else {
            unreachable!()
        }
    }
}

struct ReswordsT;
impl Rule for ReswordsT {
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        let token = parser.current_token();

        at_most_one_is_true([
            token.is(TokenKind::k__LINE__),
            token.is(TokenKind::k__FILE__),
            token.is(TokenKind::k__ENCODING__),
            token.is(TokenKind::klBEGIN),
            token.is(TokenKind::klEND),
            token.is(TokenKind::kALIAS),
            token.is(TokenKind::kAND),
            token.is(TokenKind::kBEGIN),
            token.is(TokenKind::kBREAK),
            token.is(TokenKind::kCASE),
            token.is(TokenKind::kCLASS),
            token.is(TokenKind::kDEF),
            token.is(TokenKind::kDEFINED),
            token.is(TokenKind::kDO),
            token.is(TokenKind::kELSE),
            token.is(TokenKind::kELSIF),
            token.is(TokenKind::kEND),
            token.is(TokenKind::kENSURE),
            token.is(TokenKind::kFALSE),
            token.is(TokenKind::kFOR),
            token.is(TokenKind::kIN),
            token.is(TokenKind::kMODULE),
            token.is(TokenKind::kNEXT),
            token.is(TokenKind::kNIL),
            token.is(TokenKind::kNOT),
            token.is(TokenKind::kOR),
            token.is(TokenKind::kREDO),
            token.is(TokenKind::kRESCUE),
            token.is(TokenKind::kRETRY),
            token.is(TokenKind::kRETURN),
            token.is(TokenKind::kSELF),
            token.is(TokenKind::kSUPER),
            token.is(TokenKind::kTHEN),
            token.is(TokenKind::kTRUE),
            token.is(TokenKind::kUNDEF),
            token.is(TokenKind::kWHEN),
            token.is(TokenKind::kYIELD),
            token.is(TokenKind::kIF),
            token.is(TokenKind::kUNLESS),
            token.is(TokenKind::kWHILE),
            token.is(TokenKind::kUNTIL),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if Self::starts_now(parser) {
            parser.take_token()
        } else {
            unreachable!()
        }
    }
}

pub(crate) struct Ivar;
impl Rule for Ivar {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tIVAR)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let ivar_t = parser.take_token();
        Builder::ivar(ivar_t, parser.buffer())
    }
}

pub(crate) struct Cvar;
impl Rule for Cvar {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tCVAR)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let cvar_t = parser.take_token();
        Builder::cvar(cvar_t, parser.buffer())
    }
}

pub(crate) struct Gvar;
impl Rule for Gvar {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tGVAR)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let gvar_t = parser.take_token();
        Builder::gvar(gvar_t, parser.buffer())
    }
}

struct UserVariable;
impl Rule for UserVariable {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        at_most_one_is_true([
            IdOrConstT::starts_now(parser),
            NonLocalVar::starts_now(parser),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if IdOrConstT::starts_now(parser) {
            let token = IdOrConstT::parse(parser);
            Builder::lvar(token, parser.buffer())
        } else if NonLocalVar::starts_now(parser) {
            NonLocalVar::parse(parser)
        } else {
            unreachable!()
        }
    }
}

struct NonLocalVar;
impl Rule for NonLocalVar {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        at_most_one_is_true([
            Ivar::starts_now(parser),
            Cvar::starts_now(parser),
            Gvar::starts_now(parser),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
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
impl Rule for IdOrConstT {
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        let token = parser.current_token();
        at_most_one_is_true([
            token.is(TokenKind::tIDENTIFIER),
            token.is(TokenKind::tCONSTANT),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        parser.take_token()
    }
}

#[test]
fn test_id_or_const_t() {
    use crate::loc::loc;
    use crate::token::token;

    let mut parser = Parser::new(b"foo");
    assert!(IdOrConstT::starts_now(&mut parser));
    assert_eq!(
        IdOrConstT::parse(&mut parser),
        token!(tIDENTIFIER, loc!(0, 3))
    );

    let mut parser = Parser::new(b"42");
    assert!(!IdOrConstT::starts_now(&mut parser));
}
