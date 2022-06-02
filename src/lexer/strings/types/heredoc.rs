use std::ops::ControlFlow;

use crate::lexer::{
    buffer::Buffer,
    strings::{
        action::{NextAction, StringExtendAction},
        literal::StringLiteralExtend,
        types::generate_default_string_literal_impl,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct Heredoc<'a> {
    pub(crate) supports_interpolation: bool,
    pub(crate) currently_in_interpolation: bool,
    pub(crate) ends_with: &'a [u8],
    pub(crate) interpolation_started_with_curly_level: usize,

    pub(crate) next_action: NextAction,

    pub(crate) heredoc_id_ended_at: usize,
}

generate_default_string_literal_impl!(Heredoc);

impl<'a> StringLiteralExtend<'a> for Heredoc<'a> {
    fn extend(
        &mut self,
        _buffer: &mut Buffer<'a>,
        _current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        todo!()
    }
}
