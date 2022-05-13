use crate::op_precedence::OpPrecedence;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Token<'a>(pub TokenValue<'a>, pub Loc);

impl<'a> Token<'a> {
    pub fn value(&self) -> TokenValue<'a> {
        self.0
    }

    pub fn loc(&self) -> Loc {
        self.1
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenValue<'a> {
    // Keyword tokens.
    // They are always represented by the same word in code,
    // so they have no attached value, it can be easily inferred
    kCLASS,        // `class'
    kMODULE,       // `module'
    kDEF,          // `def'
    kUNDEF,        // `undef'
    kBEGIN,        // `begin'
    kRESCUE,       // `rescue'
    kENSURE,       // `ensure'
    kEND,          // `end'
    kIF,           // `if'
    kUNLESS,       // `unless'
    kTHEN,         // `then'
    kELSIF,        // `elsif'
    kELSE,         // `else'
    kCASE,         // `case'
    kWHEN,         // `when'
    kWHILE,        // `while'
    kUNTIL,        // `until'
    kFOR,          // `for'
    kBREAK,        // `break'
    kNEXT,         // `next'
    kREDO,         // `redo'
    kRETRY,        // `retry'
    kIN,           // `in'
    kDO,           // `do'
    kDO_COND,      // `do' for condition
    kDO_BLOCK,     // `do' for block
    kDO_LAMBDA,    // `do' for lambda
    kRETURN,       // `return'
    kYIELD,        // `yield'
    kSUPER,        // `super'
    kSELF,         // `self'
    kNIL,          // `nil'
    kTRUE,         // `true'
    kFALSE,        // `false'
    kAND,          // `and'
    kOR,           // `or'
    kNOT,          // `not'
    kIF_MOD,       // `if' modifier
    kUNLESS_MOD,   // `unless' modifier
    kWHILE_MOD,    // `while' modifier
    kUNTIL_MOD,    // `until' modifier
    kRESCUE_MOD,   // `rescue' modifier
    kALIAS,        // `alias'
    kDEFINED,      // `defined?'
    klBEGIN,       // `BEGIN'
    klEND,         // `END'
    k__LINE__,     // `__LINE__'
    k__FILE__,     // `__FILE__'
    k__ENCODING__, // `__ENCODING__'

    // Variables, contain <var name>
    tIDENTIFIER(&'a [u8]), // "local variable or method"
    tFID(&'a [u8]),        // "method"
    tGVAR(&'a [u8]),       // "global variable"
    tIVAR(&'a [u8]),       // "instance variable"
    tCONSTANT(&'a [u8]),   // "constant"
    tCVAR(&'a [u8]),       // "class variable"
    tLABEL(&'a [u8]),      // "label"

    tINTEGER(&'a [u8]),        // "integer literal"
    tFLOAT(&'a [u8]),          // "float literal"
    tRATIONAL(&'a [u8]),       // "rational literal"
    tIMAGINARY(&'a [u8]),      // "imaginary literal"
    tCHAR(&'a [u8]),           // "char literal"
    tNTH_REF(&'a [u8]),        // "numbered reference"
    tBACK_REF(&'a [u8]),       // "back reference"
    tSTRING_CONTENT(&'a [u8]), // "literal content"
    tREGEXP_END(&'a [u8]),

    // Punctuation/operators
    tDOT(&'a [u8]), // "."
    /* escaped chars, should be ignored otherwise */
    tBACKSLASH(&'a [u8]),    // "backslash"
    tSP(&'a [u8]),           // "escaped space"
    tSLASH_T(&'a [u8]),      // "escaped horizontal tab"
    tSLASH_F(&'a [u8]),      // "escaped form feed"
    tSLASH_R(&'a [u8]),      // "escaped carriage return"
    tVTAB(&'a [u8]),         // "escaped vertical tab"
    tUPLUS(&'a [u8]),        // "unary+"
    tUMINUS(&'a [u8]),       // "unary-"
    tCMP(&'a [u8]),          // "<=>"
    tNEQ(&'a [u8]),          // "!="
    tGEQ(&'a [u8]),          // ">="
    tLEQ(&'a [u8]),          // "<="
    tANDOP(&'a [u8]),        // "&&"
    tOROP(&'a [u8]),         // "||"
    tMATCH(&'a [u8]),        // "=~"
    tNMATCH(&'a [u8]),       // "!~"
    tDOT2(&'a [u8]),         // ".."
    tDOT3(&'a [u8]),         // "..."
    tBDOT2(&'a [u8]),        // "(.."
    tBDOT3(&'a [u8]),        // "(..."
    tAREF(&'a [u8]),         // "[]"
    tASET(&'a [u8]),         // "[]="
    tLSHFT(&'a [u8]),        // "<<"
    tRSHFT(&'a [u8]),        // ">>"
    tANDDOT(&'a [u8]),       // "&."
    tCOLON2(&'a [u8]),       // "::"
    tCOLON3(&'a [u8]),       // ":: at EXPR_BEG"
    tOP_ASGN(&'a [u8]),      // "operator-assignment" /* +=, -=  etc. */
    tASSOC(&'a [u8]),        // "=>"
    tLPAREN,                 // "("
    tLPAREN_ARG(&'a [u8]),   // "( arg"
    tRPAREN,                 // ")"
    tLBRACK(&'a [u8]),       // "["
    tLBRACE(&'a [u8]),       // "{"
    tLBRACE_ARG(&'a [u8]),   // "{ arg"
    tDSTAR(&'a [u8]),        // "**arg"
    tAMPER(&'a [u8]),        // "&"
    tLAMBDA(&'a [u8]),       // "->"
    tSYMBEG(&'a [u8]),       // "symbol literal"
    tSTRING_BEG(&'a [u8]),   // "string begin"
    tXSTRING_BEG(&'a [u8]),  // "backtick literal"
    tREGEXP_BEG(&'a [u8]),   // "regexp literal"
    tWORDS_BEG(&'a [u8]),    // "word list"
    tQWORDS_BEG(&'a [u8]),   // "verbatim word list"
    tSYMBOLS_BEG(&'a [u8]),  // "symbol list"
    tQSYMBOLS_BEG(&'a [u8]), // "verbatim symbol list"
    tSTRING_END(&'a [u8]),   // "string end"
    tSTRING_DEND(&'a [u8]),  // "tRCURLY"
    tSTRING_DBEG(&'a [u8]),  //
    tSTRING_DVAR(&'a [u8]),  //
    tLAMBEG(&'a [u8]),       //
    tLABEL_END(&'a [u8]),    //

    tCOMMA(&'a [u8]),     // ","
    tLCURLY,              // "{ (tLCURLY)"
    tRCURLY,              // "}"
    tLBRACK2(&'a [u8]),   // "[ (tLBRACK2)"
    tPIPE(&'a [u8]),      // "|"
    tAMPER2(&'a [u8]),    // "& (tAMPER2)"
    tGT(&'a [u8]),        // ">"
    tLT(&'a [u8]),        // "<"
    tBACK_REF2(&'a [u8]), // "`"
    tCARET(&'a [u8]),     // "^"
    tLPAREN2,             // "( (tLPAREN2)"
    tRBRACK(&'a [u8]),    // "]"
    tSEMI(&'a [u8]),      // ";"
    tSPACE(&'a [u8]),     // " "
    tNL(&'a [u8]),        // "\n"
    tPERCENT(&'a [u8]),   // "%"
    tTILDE(&'a [u8]),     // "~"
    tBANG(&'a [u8]),      // "!"

    BinOp(BinOp),
    tEOF,

    // TODO: replace with diagnostics
    Error(char),

    None,
}

impl Default for TokenValue<'_> {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Loc(pub usize, pub usize);

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinOp {
    // %nonassoc, 1
    kIF_MOD,
    kUNLESS_MOD,
    kWHILE_MOD,
    kUNTIL_MOD,
    kIN,

    // %left, 2
    kOR,
    kAND,

    // %right, 3
    kNOT,

    // %nonassoc, 4
    kDEFINED,

    // %right, 5
    tEQL,
    tOP_ASGN,

    // %left, 6
    kRESCUE_MOD,

    // %right, 7
    tEH,
    tCOLON,

    // %nonassoc, 8
    tDOT2,
    tDOT3,
    tBDOT2,
    tBDOT3,

    // %left, 9
    tOROP,

    // %left, 10
    tANDOP,

    // %nonassoc, 11
    tCMP,
    tEQ,
    tEQQ,
    tNEQ,
    tMATCH,
    tNMATCH,

    // %left, 12
    tGT,
    tGEQ,
    tLT,
    tLEQ,

    // %left, 13
    tPIPE,
    tCARET,

    // %left, 14
    tAMPER,

    // %left, 15
    tLSHFT,
    tRSHFT,

    // %left, 16
    tPLUS,
    tMINUS,

    // %left, 17
    tSTAR,
    tDIVIDE,
    tPERCENT,

    // %right, 18
    tUMINUS_NUM,
    tUMINUS,

    // %right, 19
    tPOW,

    // %right, 20
    tBANG,
    tTILDE,
    tUPLUS,
}

impl BinOp {
    pub(crate) fn precedence(&self) -> OpPrecedence {
        use BinOp::*;

        match self {
            kIF_MOD | kUNLESS_MOD | kWHILE_MOD | kUNTIL_MOD | kIN => OpPrecedence::None(1),
            kOR | kAND => OpPrecedence::Left(1),
            kNOT => OpPrecedence::Right(3),
            kDEFINED => OpPrecedence::None(4),
            tEQL | tOP_ASGN => OpPrecedence::Right(5),
            kRESCUE_MOD => OpPrecedence::Left(6),
            tEH | tCOLON => OpPrecedence::Right(7),
            tDOT2 | tDOT3 | tBDOT2 | tBDOT3 => OpPrecedence::None(8),
            tOROP => OpPrecedence::Left(9),
            tANDOP => OpPrecedence::Left(10),
            tCMP | tEQ | tEQQ | tNEQ | tMATCH | tNMATCH => OpPrecedence::None(11),
            tGT | tGEQ | tLT | tLEQ => OpPrecedence::Left(12),
            tPIPE | tCARET => OpPrecedence::Left(13),
            tAMPER => OpPrecedence::Left(14),
            tLSHFT | tRSHFT => OpPrecedence::Left(15),
            tPLUS | tMINUS => OpPrecedence::Left(16),
            tSTAR | tDIVIDE | tPERCENT => OpPrecedence::Left(17),
            tUMINUS_NUM | tUMINUS => OpPrecedence::Right(18),
            tPOW => OpPrecedence::Right(19),
            tBANG | tTILDE | tUPLUS => OpPrecedence::Right(20),
        }
    }
}
