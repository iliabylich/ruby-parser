use crate::{buffer::Buffer, Loc};

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub loc: Loc,
    pub value: Option<TokenValue>,
}

impl Token {
    pub fn is(&self, other: TokenKind) -> bool {
        self.kind == other
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
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

    tINTEGER,        // "integer literal"
    tFLOAT,          // "float literal"
    tRATIONAL,       // "rational literal"
    tIMAGINARY,      // "imaginary literal"
    tCHAR,           // "char literal"
    tNTH_REF,        // "numbered reference"
    tBACK_REF,       // "back reference"
    tSTRING_CONTENT, // "literal content"
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

    tUNINITIALIZED,

    #[cfg(test)]
    tTEST_TOKEN,
}

impl Default for TokenKind {
    fn default() -> Self {
        Self::tUNINITIALIZED
    }
}

// Everything that doesn't involve escaping can be taken directly
// from buffer, that's why it's not stored here.
// Simply do `buffer.slice(token.loc)` to get a byte slice of the token
#[derive(Debug, Eq, Clone, Copy)]
pub enum TokenValue {
    // used for trivial single-byte escape sequences like `\xFF`
    UnescapedByte(u8),

    // used for multibyte characters and escape sequences like `\u1234`
    UnescapedChar(char),

    // used for "wide" \u escape sequences like `\u{1234 5678}`
    //
    // This variant is special as it requires boxing.
    // Instead, we write all unescaped multi-char sequences
    // to parser.buffer.unescaped_chars
    //
    UnescapedChars { loc: Loc, buffer: *const Buffer },
}

impl From<u8> for TokenValue {
    fn from(byte: u8) -> Self {
        Self::UnescapedByte(byte)
    }
}

impl From<char> for TokenValue {
    fn from(c: char) -> Self {
        Self::UnescapedChar(c)
    }
}

impl TokenValue {
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        match self {
            TokenValue::UnescapedByte(byte) => vec![*byte],
            TokenValue::UnescapedChar(c) => {
                let mut buf = vec![0_u8; c.len_utf8()];
                c.encode_utf8(&mut buf);
                buf
            }
            TokenValue::UnescapedChars { loc, buffer } => {
                let slice = unsafe { buffer.as_ref().unwrap() }
                    .unescaped_slice_at(loc.start, loc.end)
                    .unwrap_or_else(|| {
                        panic!("bug: missing unescaped slice {}..{}", loc.start, loc.end)
                    });
                slice.iter().cloned().collect()
            }
        }
    }
}

impl PartialEq for TokenValue {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes() == other.to_bytes()
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
        use crate::token::{TokenKind::*, TokenValue};
        crate::token::Token {
            kind: $kind,
            loc: $loc,
            value: Some(TokenValue::from($value)),
        }
    }};
}
pub(crate) use token;
