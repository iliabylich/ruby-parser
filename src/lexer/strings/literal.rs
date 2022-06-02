use std::ops::ControlFlow;

use crate::lexer::{
    buffer::Buffer,
    strings::{
        action::StringExtendAction,
        types::{Heredoc, Regexp, String, SymArray, Symbol, WordArray},
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
    String(String<'a>),
    Symbol(Symbol<'a>),
    Heredoc(Heredoc<'a>),
    Regexp(Regexp<'a>),
    SymArray(SymArray<'a>),
    WordArray(WordArray<'a>),
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
            Self::SymArray(sym_array) => sym_array.extend(buffer, current_curly_nest),
            Self::WordArray(word_array) => word_array.extend(buffer, current_curly_nest),
        }
    }
}

macro_rules! for_each_branch_pick_attribute {
    ($this:expr, $attr:ident) => {
        match $this {
            Self::String(String { $attr, .. })
            | Self::Symbol(Symbol { $attr, .. })
            | Self::Heredoc(Heredoc { $attr, .. })
            | Self::Regexp(Regexp { $attr, .. })
            | Self::SymArray(SymArray { $attr, .. })
            | Self::WordArray(WordArray { $attr, .. }) => $attr,
        }
    };
}

impl<'a> StringLiteral<'a> {
    pub(crate) fn string() -> Self {
        Self::String(String::default())
    }

    pub(crate) fn symbol() -> Self {
        Self::Symbol(Symbol::default())
    }

    pub(crate) fn heredoc(heredoc_id_ended_at: usize) -> Self {
        Self::Heredoc(Heredoc {
            heredoc_id_ended_at,
            ..Heredoc::default()
        })
    }

    pub(crate) fn regexp() -> Self {
        Self::Regexp(Regexp::default())
    }

    pub(crate) fn sym_array() -> Self {
        Self::SymArray(SymArray::default())
    }

    pub(crate) fn word_array() -> Self {
        Self::WordArray(WordArray::default())
    }

    pub(crate) fn stop_interpolation(&mut self) {
        let currently_in_interpolation =
            for_each_branch_pick_attribute!(self, currently_in_interpolation);

        *currently_in_interpolation = false;
    }

    pub(crate) fn start_interpolation(&mut self) {
        let currently_in_interpolation =
            for_each_branch_pick_attribute!(self, currently_in_interpolation);

        *currently_in_interpolation = true;
    }

    pub(crate) fn with_interpolation_support(mut self, value: bool) -> Self {
        let supports_interpolation =
            for_each_branch_pick_attribute!(&mut self, supports_interpolation);
        *supports_interpolation = value;
        self
    }

    pub(crate) fn with_ending(mut self, value: &'a [u8]) -> Self {
        let ends_with = for_each_branch_pick_attribute!(&mut self, ends_with);
        *ends_with = value;
        self
    }

    pub(crate) fn with_curly_level(mut self, value: usize) -> Self {
        let interpolation_started_with_curly_level =
            for_each_branch_pick_attribute!(&mut self, interpolation_started_with_curly_level);
        *interpolation_started_with_curly_level = value;
        self
    }
}
