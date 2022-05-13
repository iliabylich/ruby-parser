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
    kIF_MOD,       // `if` modifier
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
    tDOT, // "."
    /* escaped chars, should be ignored otherwise */
    tBACKSLASH,              // "backslash"
    tSP,                     // "escaped space"
    tSLASH_T,                // "escaped horizontal tab"
    tSLASH_F,                // "escaped form feed"
    tSLASH_R,                // "escaped carriage return"
    tVTAB,                   // "escaped vertical tab"
    tUPLUS,                  // "unary+"
    tUMINUS,                 // "unary-"
    tUMINUS_NUM,             // "unary-" before number literal
    tCMP,                    // "<=>"
    tNEQ,                    // "!="
    tGEQ,                    // ">="
    tLEQ,                    // "<="
    tANDOP,                  // "&&"
    tOROP,                   // "||"
    tMATCH,                  // "=~"
    tNMATCH,                 // "!~"
    tDOT2,                   // ".."
    tDOT3,                   // "..."
    tBDOT2,                  // "(.."
    tBDOT3,                  // "(..."
    tAREF,                   // "[]"
    tASET,                   // "[]="
    tLSHFT,                  // "<<"
    tRSHFT,                  // ">>"
    tANDDOT,                 // "&."
    tCOLON,                  // ":"
    tCOLON2,                 // "::"
    tOP_ASGN,                // "operator-assignment" /* +=, -=  etc. */
    tASSOC,                  // "=>"
    tLPAREN,                 // "("
    tRPAREN,                 // ")"
    tLBRACK,                 // "["
    tRBRACK,                 // "]"
    tLBRACE,                 // "{"
    tDSTAR,                  // "**arg"
    tAMPER,                  // "&"
    tLAMBDA,                 // "->"
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

    tCOMMA,   // ","
    tLCURLY,  // "{"
    tRCURLY,  // "}"
    tEQL,     // "="
    tPIPE,    // "|"
    tGT,      // ">"
    tLT,      // "<"
    tCARET,   // "^"
    tSEMI,    // ";"
    tSPACE,   // " "
    tNL,      // "\n"
    tPERCENT, // "%"
    tTILDE,   // "~"
    tBANG,    // "!"
    tPLUS,    // "+"
    tMINUS,   // "-"
    tPOW,     // "**"
    tSTAR,    // "*"
    tDIVIDE,  // "/"
    tEQ,      // "=="
    tEQQ,     // "==="
    tEH,      // "?"

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

impl std::convert::TryFrom<TokenValue<'_>> for BinOp {
    type Error = ();

    fn try_from(token: TokenValue<'_>) -> Result<Self, Self::Error> {
        match token {
            TokenValue::kIF_MOD => Ok(BinOp::kIF_MOD),
            TokenValue::kUNLESS_MOD => Ok(BinOp::kUNLESS_MOD),
            TokenValue::kWHILE_MOD => Ok(BinOp::kWHILE_MOD),
            TokenValue::kUNTIL_MOD => Ok(BinOp::kUNTIL_MOD),
            TokenValue::kIN => Ok(BinOp::kIN),
            TokenValue::kOR => Ok(BinOp::kOR),
            TokenValue::kAND => Ok(BinOp::kAND),
            TokenValue::kNOT => Ok(BinOp::kNOT),
            TokenValue::kDEFINED => Ok(BinOp::kDEFINED),
            TokenValue::tEQL => Ok(BinOp::tEQL),
            TokenValue::tOP_ASGN => Ok(BinOp::tOP_ASGN),
            TokenValue::kRESCUE_MOD => Ok(BinOp::kRESCUE_MOD),
            TokenValue::tEH => Ok(BinOp::tEH),
            TokenValue::tCOLON => Ok(BinOp::tCOLON),
            TokenValue::tDOT2 => Ok(BinOp::tDOT2),
            TokenValue::tDOT3 => Ok(BinOp::tDOT3),
            TokenValue::tBDOT2 => Ok(BinOp::tBDOT2),
            TokenValue::tBDOT3 => Ok(BinOp::tBDOT3),
            TokenValue::tOROP => Ok(BinOp::tOROP),
            TokenValue::tANDOP => Ok(BinOp::tANDOP),
            TokenValue::tCMP => Ok(BinOp::tCMP),
            TokenValue::tEQ => Ok(BinOp::tEQ),
            TokenValue::tEQQ => Ok(BinOp::tEQQ),
            TokenValue::tNEQ => Ok(BinOp::tNEQ),
            TokenValue::tMATCH => Ok(BinOp::tMATCH),
            TokenValue::tNMATCH => Ok(BinOp::tNMATCH),
            TokenValue::tGT => Ok(BinOp::tGT),
            TokenValue::tGEQ => Ok(BinOp::tGEQ),
            TokenValue::tLT => Ok(BinOp::tLT),
            TokenValue::tLEQ => Ok(BinOp::tLEQ),
            TokenValue::tPIPE => Ok(BinOp::tPIPE),
            TokenValue::tCARET => Ok(BinOp::tCARET),
            TokenValue::tAMPER => Ok(BinOp::tAMPER),
            TokenValue::tLSHFT => Ok(BinOp::tLSHFT),
            TokenValue::tRSHFT => Ok(BinOp::tRSHFT),
            TokenValue::tPLUS => Ok(BinOp::tPLUS),
            TokenValue::tMINUS => Ok(BinOp::tMINUS),
            TokenValue::tSTAR => Ok(BinOp::tSTAR),
            TokenValue::tDIVIDE => Ok(BinOp::tDIVIDE),
            TokenValue::tPERCENT => Ok(BinOp::tPERCENT),
            TokenValue::tUMINUS_NUM => Ok(BinOp::tUMINUS_NUM),
            TokenValue::tUMINUS => Ok(BinOp::tUMINUS),
            TokenValue::tPOW => Ok(BinOp::tPOW),
            TokenValue::tBANG => Ok(BinOp::tBANG),
            TokenValue::tTILDE => Ok(BinOp::tTILDE),
            TokenValue::tUPLUS => Ok(BinOp::tUPLUS),
            _ => Err(()),
        }
    }
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
