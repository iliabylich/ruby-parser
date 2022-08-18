use std::ops::ControlFlow;

use crate::{
    buffer::BufferWithCursor,
    lexer::strings::{
        action::StringExtendAction,
        types::{
            Heredoc, Regexp, StringInterp, StringPlain, SymbolInterp, SymbolPlain, WordsInterp,
            WordsPlain,
        },
    },
};

pub(crate) trait StringLiteralExtend {
    fn extend(
        &mut self,
        buffer: &mut BufferWithCursor,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum StringLiteral {
    StringInterp(StringInterp),
    StringPlain(StringPlain),

    WordsInterp(WordsInterp),
    WordsPlain(WordsPlain),

    SymbolInterp(SymbolInterp),
    SymbolPlain(SymbolPlain),

    Heredoc(Heredoc),
    Regexp(Regexp),
}

impl StringLiteralExtend for StringLiteral {
    fn extend(
        &mut self,
        buffer: &mut BufferWithCursor,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        match self {
            Self::StringInterp(string) => string.extend(buffer, current_curly_nest),
            Self::StringPlain(string) => string.extend(buffer, current_curly_nest),
            Self::SymbolInterp(symbol) => symbol.extend(buffer, current_curly_nest),
            Self::SymbolPlain(symbol) => symbol.extend(buffer, current_curly_nest),
            Self::Heredoc(heredoc) => heredoc.extend(buffer, current_curly_nest),
            Self::Regexp(regexp) => regexp.extend(buffer, current_curly_nest),
            Self::WordsInterp(words) => words.extend(buffer, current_curly_nest),
            Self::WordsPlain(words) => words.extend(buffer, current_curly_nest),
        }
    }
}
