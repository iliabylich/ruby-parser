use crate::token::TokenValue;

#[derive(Clone, Copy)]
pub(crate) struct ReservedWord<'a> {
    name: &'static [u8],
    pub(crate) token_value: TokenValue<'a>,
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
            token_value: TokenValue::kDO,
        },
        ReservedWord {
            name: b"if",
            token_value: TokenValue::kIF,
        },
        ReservedWord {
            name: b"in",
            token_value: TokenValue::kIN,
        },
        ReservedWord {
            name: b"or",
            token_value: TokenValue::kOR,
        },
    ],
    // size = 3
    &[
        ReservedWord {
            name: b"END",
            token_value: TokenValue::klEND,
        },
        ReservedWord {
            name: b"and",
            token_value: TokenValue::kAND,
        },
        ReservedWord {
            name: b"def",
            token_value: TokenValue::kDEF,
        },
        ReservedWord {
            name: b"end",
            token_value: TokenValue::kEND,
        },
        ReservedWord {
            name: b"for",
            token_value: TokenValue::kFOR,
        },
        ReservedWord {
            name: b"nil",
            token_value: TokenValue::kNIL,
        },
        ReservedWord {
            name: b"not",
            token_value: TokenValue::kNOT,
        },
    ],
    // size = 4
    &[
        ReservedWord {
            name: b"case",
            token_value: TokenValue::kCASE,
        },
        ReservedWord {
            name: b"else",
            token_value: TokenValue::kELSE,
        },
        ReservedWord {
            name: b"next",
            token_value: TokenValue::kNEXT,
        },
        ReservedWord {
            name: b"redo",
            token_value: TokenValue::kREDO,
        },
        ReservedWord {
            name: b"self",
            token_value: TokenValue::kSELF,
        },
        ReservedWord {
            name: b"then",
            token_value: TokenValue::kTHEN,
        },
        ReservedWord {
            name: b"true",
            token_value: TokenValue::kTRUE,
        },
        ReservedWord {
            name: b"when",
            token_value: TokenValue::kWHEN,
        },
    ],
    // size = 5
    &[
        ReservedWord {
            name: b"BEGIN",
            token_value: TokenValue::klBEGIN,
        },
        ReservedWord {
            name: b"alias",
            token_value: TokenValue::kALIAS,
        },
        ReservedWord {
            name: b"begin",
            token_value: TokenValue::kBEGIN,
        },
        ReservedWord {
            name: b"break",
            token_value: TokenValue::kBREAK,
        },
        ReservedWord {
            name: b"class",
            token_value: TokenValue::kCLASS,
        },
        ReservedWord {
            name: b"elsif",
            token_value: TokenValue::kELSIF,
        },
        ReservedWord {
            name: b"false",
            token_value: TokenValue::kFALSE,
        },
        ReservedWord {
            name: b"retry",
            token_value: TokenValue::kRETRY,
        },
        ReservedWord {
            name: b"super",
            token_value: TokenValue::kSUPER,
        },
        ReservedWord {
            name: b"undef",
            token_value: TokenValue::kUNDEF,
        },
        ReservedWord {
            name: b"until",
            token_value: TokenValue::kUNTIL,
        },
        ReservedWord {
            name: b"while",
            token_value: TokenValue::kWHILE,
        },
        ReservedWord {
            name: b"yield",
            token_value: TokenValue::kYIELD,
        },
    ],
    // size = 6
    &[
        ReservedWord {
            name: b"ensure",
            token_value: TokenValue::kENSURE,
        },
        ReservedWord {
            name: b"module",
            token_value: TokenValue::kMODULE,
        },
        ReservedWord {
            name: b"rescue",
            token_value: TokenValue::kRESCUE,
        },
        ReservedWord {
            name: b"return",
            token_value: TokenValue::kRETURN,
        },
        ReservedWord {
            name: b"unless",
            token_value: TokenValue::kUNLESS,
        },
    ],
    // size = 7
    &[],
    // size = 8
    &[
        ReservedWord {
            name: b"__FILE__",
            token_value: TokenValue::k__FILE__,
        },
        ReservedWord {
            name: b"__LINE__",
            token_value: TokenValue::k__LINE__,
        },
        ReservedWord {
            name: b"defined?",
            token_value: TokenValue::kDEFINED,
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
        token_value: TokenValue::k__ENCODING__,
    }],
];

pub(crate) fn find_reserved_word(tok: &[u8]) -> Option<ReservedWord> {
    let bucket = RESERVED_WORDS.get(tok.len())?;
    let idx = bucket.binary_search_by(|e| e.name.cmp(tok)).ok()?;

    Some(bucket[idx])
}
