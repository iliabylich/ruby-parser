use crate::token::TokenKind;

#[derive(Clone)]
pub(crate) struct ReservedWord<'a> {
    name: &'static [u8],
    pub(crate) token_value: TokenKind<'a>,
}

pub(crate) const RESERVED_WORDS: &[&[ReservedWord]] = &[
    // size = 0
    &[],
    // size = 1
    &[],
    // size = 2
    &[
        ReservedWord {
            name: b"do",
            token_value: TokenKind::kDO,
        },
        ReservedWord {
            name: b"if",
            token_value: TokenKind::kIF,
        },
        ReservedWord {
            name: b"in",
            token_value: TokenKind::kIN,
        },
        ReservedWord {
            name: b"or",
            token_value: TokenKind::kOR,
        },
    ],
    // size = 3
    &[
        ReservedWord {
            name: b"END",
            token_value: TokenKind::klEND,
        },
        ReservedWord {
            name: b"and",
            token_value: TokenKind::kAND,
        },
        ReservedWord {
            name: b"def",
            token_value: TokenKind::kDEF,
        },
        ReservedWord {
            name: b"end",
            token_value: TokenKind::kEND,
        },
        ReservedWord {
            name: b"for",
            token_value: TokenKind::kFOR,
        },
        ReservedWord {
            name: b"nil",
            token_value: TokenKind::kNIL,
        },
        ReservedWord {
            name: b"not",
            token_value: TokenKind::kNOT,
        },
    ],
    // size = 4
    &[
        ReservedWord {
            name: b"case",
            token_value: TokenKind::kCASE,
        },
        ReservedWord {
            name: b"else",
            token_value: TokenKind::kELSE,
        },
        ReservedWord {
            name: b"next",
            token_value: TokenKind::kNEXT,
        },
        ReservedWord {
            name: b"redo",
            token_value: TokenKind::kREDO,
        },
        ReservedWord {
            name: b"self",
            token_value: TokenKind::kSELF,
        },
        ReservedWord {
            name: b"then",
            token_value: TokenKind::kTHEN,
        },
        ReservedWord {
            name: b"true",
            token_value: TokenKind::kTRUE,
        },
        ReservedWord {
            name: b"when",
            token_value: TokenKind::kWHEN,
        },
    ],
    // size = 5
    &[
        ReservedWord {
            name: b"BEGIN",
            token_value: TokenKind::klBEGIN,
        },
        ReservedWord {
            name: b"alias",
            token_value: TokenKind::kALIAS,
        },
        ReservedWord {
            name: b"begin",
            token_value: TokenKind::kBEGIN,
        },
        ReservedWord {
            name: b"break",
            token_value: TokenKind::kBREAK,
        },
        ReservedWord {
            name: b"class",
            token_value: TokenKind::kCLASS,
        },
        ReservedWord {
            name: b"elsif",
            token_value: TokenKind::kELSIF,
        },
        ReservedWord {
            name: b"false",
            token_value: TokenKind::kFALSE,
        },
        ReservedWord {
            name: b"retry",
            token_value: TokenKind::kRETRY,
        },
        ReservedWord {
            name: b"super",
            token_value: TokenKind::kSUPER,
        },
        ReservedWord {
            name: b"undef",
            token_value: TokenKind::kUNDEF,
        },
        ReservedWord {
            name: b"until",
            token_value: TokenKind::kUNTIL,
        },
        ReservedWord {
            name: b"while",
            token_value: TokenKind::kWHILE,
        },
        ReservedWord {
            name: b"yield",
            token_value: TokenKind::kYIELD,
        },
    ],
    // size = 6
    &[
        ReservedWord {
            name: b"ensure",
            token_value: TokenKind::kENSURE,
        },
        ReservedWord {
            name: b"module",
            token_value: TokenKind::kMODULE,
        },
        ReservedWord {
            name: b"rescue",
            token_value: TokenKind::kRESCUE,
        },
        ReservedWord {
            name: b"return",
            token_value: TokenKind::kRETURN,
        },
        ReservedWord {
            name: b"unless",
            token_value: TokenKind::kUNLESS,
        },
    ],
    // size = 7
    &[],
    // size = 8
    &[
        ReservedWord {
            name: b"__FILE__",
            token_value: TokenKind::k__FILE__,
        },
        ReservedWord {
            name: b"__LINE__",
            token_value: TokenKind::k__LINE__,
        },
        ReservedWord {
            name: b"defined?",
            token_value: TokenKind::kDEFINED,
        },
    ],
    // size = 9
    &[],
    // size = 10
    &[],
    // size = 11
    &[],
    // size = 12
    &[ReservedWord {
        name: b"__ENCODING__",
        token_value: TokenKind::k__ENCODING__,
    }],
];

pub(crate) fn find_reserved_word(tok: &[u8]) -> Option<ReservedWord> {
    let bucket = RESERVED_WORDS.get(tok.len())?;
    let idx = bucket.binary_search_by(|e| e.name.cmp(tok)).ok()?;

    Some(bucket[idx].clone())
}
