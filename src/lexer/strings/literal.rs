use std::ops::ControlFlow;

use crate::{
    buffer::BufferWithCursor,
    lexer::strings::{
        action::StringExtendAction,
        types::{Array, Heredoc, Regexp, StringInterp, StringPlain, Symbol},
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

    Symbol(Symbol),
    Heredoc(Heredoc),
    Regexp(Regexp),
    Array(Array),
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
            Self::Symbol(symbol) => symbol.extend(buffer, current_curly_nest),
            Self::Heredoc(heredoc) => heredoc.extend(buffer, current_curly_nest),
            Self::Regexp(regexp) => regexp.extend(buffer, current_curly_nest),
            Self::Array(sym_array) => sym_array.extend(buffer, current_curly_nest),
        }
    }
}
