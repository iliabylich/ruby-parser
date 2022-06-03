use std::ops::ControlFlow;

use crate::lexer::{
    buffer::Buffer,
    strings::{
        action::StringExtendAction,
        types::{Array, Heredoc, Regexp, String, Symbol},
    },
};

pub(crate) trait StringLiteralExtend<'a> {
    fn extend(
        &mut self,
        buffer: &mut Buffer<'a>,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction>;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum StringLiteral<'a> {
    String(String),
    Symbol(Symbol),
    Heredoc(Heredoc<'a>),
    Regexp(Regexp),
    Array(Array),
}

impl<'a> StringLiteralExtend<'a> for StringLiteral<'a> {
    fn extend(
        &mut self,
        buffer: &mut Buffer<'a>,
        current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        match self {
            Self::String(string) => string.extend(buffer, current_curly_nest),
            Self::Symbol(symbol) => symbol.extend(buffer, current_curly_nest),
            Self::Heredoc(heredoc) => heredoc.extend(buffer, current_curly_nest),
            Self::Regexp(regexp) => regexp.extend(buffer, current_curly_nest),
            Self::Array(sym_array) => sym_array.extend(buffer, current_curly_nest),
        }
    }
}
