use std::ops::ControlFlow;

use crate::lexer::{
    buffer::BufferWithCursor,
    strings::{
        action::StringExtendAction,
        types::{Array, Heredoc, Regexp, StringInterp, StringNoInterp, Symbol},
    },
};

pub(crate) trait StringLiteralExtend<'a> {
    fn extend(
        &mut self,
        buffer: &mut BufferWithCursor<'a>,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction<'a>>;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum StringLiteral<'a> {
    StringInterp(StringInterp),
    StringNoInterp(StringNoInterp),

    Symbol(Symbol),
    Heredoc(Heredoc<'a>),
    Regexp(Regexp),
    Array(Array),
}

impl<'a> StringLiteralExtend<'a> for StringLiteral<'a> {
    fn extend(
        &mut self,
        buffer: &mut BufferWithCursor<'a>,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction<'a>> {
        match self {
            Self::StringInterp(string) => string.extend(buffer, current_curly_nest),
            Self::StringNoInterp(string) => string.extend(buffer, current_curly_nest),
            Self::Symbol(symbol) => symbol.extend(buffer, current_curly_nest),
            Self::Heredoc(heredoc) => heredoc.extend(buffer, current_curly_nest),
            Self::Regexp(regexp) => regexp.extend(buffer, current_curly_nest),
            Self::Array(sym_array) => sym_array.extend(buffer, current_curly_nest),
        }
    }
}
