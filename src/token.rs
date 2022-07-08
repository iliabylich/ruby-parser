use crate::{string_content::StringContent, Loc};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub loc: Loc,
    pub value: Option<Vec<u8>>,
}

impl<'a> Token<'a> {
    pub fn kind(&self) -> &TokenKind<'a> {
        &self.kind
    }

    pub fn is(&self, other: TokenKind) -> bool {
        self.kind == other
    }

    pub fn loc(&self) -> Loc {
        self.loc
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind<'a> {
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
    tIDENTIFIER, // "local variable or method"
    tFID,        // "method"
    tGVAR,       // "global variable"
    tIVAR,       // "instance variable"
    tCONSTANT,   // "constant"
    tCVAR,       // "class variable"
    tLABEL,      // "label"

    tINTEGER,                           // "integer literal"
    tFLOAT,                             // "float literal"
    tRATIONAL,                          // "rational literal"
    tIMAGINARY,                         // "imaginary literal"
    tCHAR,                              // "char literal"
    tNTH_REF,                           // "numbered reference"
    tBACK_REF,                          // "back reference"
    tSTRING_CONTENT(StringContent<'a>), // "literal content"
    tREGEXP_END,

    // Punctuation/operators
    tDOT, // "."
    /* escaped chars, should be ignored otherwise */
    tBACKSLASH,    // "backslash"
    tSP,           // "escaped space"
    tSLASH_T,      // "escaped horizontal tab"
    tSLASH_F,      // "escaped form feed"
    tSLASH_R,      // "escaped carriage return"
    tVTAB,         // "escaped vertical tab"
    tUPLUS,        // "unary+"
    tUMINUS,       // "unary-"
    tUMINUS_NUM,   // "unary-" before number literal
    tCMP,          // "<=>"
    tNEQ,          // "!="
    tGEQ,          // ">="
    tLEQ,          // "<="
    tANDOP,        // "&&"
    tOROP,         // "||"
    tMATCH,        // "=~"
    tNMATCH,       // "!~"
    tDOT2,         // ".."
    tDOT3,         // "..."
    tBDOT2,        // "(.."
    tBDOT3,        // "(..."
    tAREF,         // "[]"
    tASET,         // "[]="
    tLSHFT,        // "<<"
    tRSHFT,        // ">>"
    tANDDOT,       // "&."
    tCOLON,        // ":"
    tCOLON2,       // "::"
    tOP_ASGN,      // "operator-assignment" /* +=, -=  etc. */
    tASSOC,        // "=>"
    tLPAREN,       // "("
    tRPAREN,       // ")"
    tLBRACK,       // "["
    tRBRACK,       // "]"
    tDSTAR,        // "**arg"
    tAMPER,        // "&"
    tLAMBDA,       // "->"
    tSYMBEG,       // "symbol literal start"
    tDSYMBEG,      // "dynamic symbol literal start"
    tSTRING_BEG,   // "string begin"
    tHEREDOC_BEG,  // "heredoc begin"
    tDSTRING_BEG,  // "dynamic string begin"
    tXSTRING_BEG,  // "backtick literal"
    tXHEREDOC_BEG, // "backtick heredoc"
    tREGEXP_BEG,   // "regexp literal"
    tWORDS_BEG,    // "word list"
    tQWORDS_BEG,   // "verbatim word list"
    tSYMBOLS_BEG,  // "symbol list"
    tQSYMBOLS_BEG, // "verbatim symbol list"
    tSTRING_END,   // "string end"
    tSTRING_DBEG,  // "#{"
    tSTRING_DEND,  // "tRCURLY"
    tSTRING_DVAR,  // "#" (in case of #@var / #@@var / #$var)
    tLAMBEG,       //
    tLABEL_END,    //

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

    tEMBEDDED_COMMENT_START, // "=begin"
    tCOMMENT,                // comment content
    tEMBEDDED_COMMENT_END,   // "=end"

    tEOF,

    // TODO: replace with diagnostics
    Error(char),

    tUNINITIALIZED,

    #[cfg(test)]
    tTEST_TOKEN,
}

impl Default for TokenKind<'_> {
    fn default() -> Self {
        Self::tUNINITIALIZED
    }
}

macro_rules! token {
    ($kind:expr, $loc:expr) => {{
        #[allow(unused_imports)]
        use crate::token::TokenKind::*;
        crate::token::Token {
            kind: $kind,
            loc: $loc,
            value: None,
        }
    }};
    ($kind:expr, $loc:expr, $value:expr) => {{
        #[allow(unused_imports)]
        use crate::token::TokenKind::*;
        crate::token::Token {
            kind: $kind,
            loc: $loc,
            value: Some($value),
        }
    }};
}
pub(crate) use token;
