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
    tIDENTIFIER(&'a str), // "local variable or method"
    tFID(&'a str),        // "method"
    tGVAR(&'a str),       // "global variable"
    tIVAR(&'a str),       // "instance variable"
    tCONSTANT(&'a str),   // "constant"
    tCVAR(&'a str),       // "class variable"
    tLABEL(&'a str),      // "label"

    tINTEGER(&'a str),        // "integer literal"
    tFLOAT(&'a str),          // "float literal"
    tRATIONAL(&'a str),       // "rational literal"
    tIMAGINARY(&'a str),      // "imaginary literal"
    tCHAR(&'a str),           // "char literal"
    tNTH_REF(&'a str),        // "numbered reference"
    tBACK_REF(&'a str),       // "back reference"
    tSTRING_CONTENT(&'a str), // "literal content"
    tREGEXP_END(&'a str),

    // Punctuation/operators
    tDOT(&'a str), // "."
    /* escaped chars, should be ignored otherwise */
    tBACKSLASH(&'a str),    // "backslash"
    tSP(&'a str),           // "escaped space"
    tSLASH_T(&'a str),      // "escaped horizontal tab"
    tSLASH_F(&'a str),      // "escaped form feed"
    tSLASH_R(&'a str),      // "escaped carriage return"
    tVTAB(&'a str),         // "escaped vertical tab"
    tUPLUS(&'a str),        // "unary+"
    tUMINUS(&'a str),       // "unary-"
    tCMP(&'a str),          // "<=>"
    tNEQ(&'a str),          // "!="
    tGEQ(&'a str),          // ">="
    tLEQ(&'a str),          // "<="
    tANDOP(&'a str),        // "&&"
    tOROP(&'a str),         // "||"
    tMATCH(&'a str),        // "=~"
    tNMATCH(&'a str),       // "!~"
    tDOT2(&'a str),         // ".."
    tDOT3(&'a str),         // "..."
    tBDOT2(&'a str),        // "(.."
    tBDOT3(&'a str),        // "(..."
    tAREF(&'a str),         // "[]"
    tASET(&'a str),         // "[]="
    tLSHFT(&'a str),        // "<<"
    tRSHFT(&'a str),        // ">>"
    tANDDOT(&'a str),       // "&."
    tCOLON2(&'a str),       // "::"
    tCOLON3(&'a str),       // ":: at EXPR_BEG"
    tOP_ASGN(&'a str),      // "operator-assignment" /* +=, -=  etc. */
    tASSOC(&'a str),        // "=>"
    tLPAREN,                // "("
    tLPAREN_ARG(&'a str),   // "( arg"
    tRPAREN,                // ")"
    tLBRACK(&'a str),       // "["
    tLBRACE(&'a str),       // "{"
    tLBRACE_ARG(&'a str),   // "{ arg"
    tDSTAR(&'a str),        // "**arg"
    tAMPER(&'a str),        // "&"
    tLAMBDA(&'a str),       // "->"
    tSYMBEG(&'a str),       // "symbol literal"
    tSTRING_BEG(&'a str),   // "string begin"
    tXSTRING_BEG(&'a str),  // "backtick literal"
    tREGEXP_BEG(&'a str),   // "regexp literal"
    tWORDS_BEG(&'a str),    // "word list"
    tQWORDS_BEG(&'a str),   // "verbatim word list"
    tSYMBOLS_BEG(&'a str),  // "symbol list"
    tQSYMBOLS_BEG(&'a str), // "verbatim symbol list"
    tSTRING_END(&'a str),   // "string end"
    tSTRING_DEND(&'a str),  // "tRCURLY"
    tSTRING_DBEG(&'a str),  //
    tSTRING_DVAR(&'a str),  //
    tLAMBEG(&'a str),       //
    tLABEL_END(&'a str),    //

    tCOMMA(&'a str),     // ","
    tLCURLY(&'a str),    // "{ (tLCURLY)"
    tRCURLY(&'a str),    // "}"
    tLBRACK2(&'a str),   // "[ (tLBRACK2)"
    tPIPE(&'a str),      // "|"
    tAMPER2(&'a str),    // "& (tAMPER2)"
    tGT(&'a str),        // ">"
    tLT(&'a str),        // "<"
    tBACK_REF2(&'a str), // "`"
    tCARET(&'a str),     // "^"
    tLPAREN2,            // "( (tLPAREN2)"
    tRBRACK(&'a str),    // "]"
    tSEMI(&'a str),      // ";"
    tSPACE(&'a str),     // " "
    tNL(&'a str),        // "\n"
    tPERCENT(&'a str),   // "%"
    tTILDE(&'a str),     // "~"
    tBANG(&'a str),      // "!"

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
