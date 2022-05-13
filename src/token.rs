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
