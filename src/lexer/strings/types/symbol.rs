use std::ops::ControlFlow;

use crate::lexer::{
    buffer::Buffer,
    strings::{
        action::{NextAction, StringExtendAction},
        handlers::handle_eof,
        literal::StringLiteralExtend,
        types::generate_default_string_literal_impl,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct Symbol<'a> {
    pub(crate) supports_interpolation: bool,
    pub(crate) currently_in_interpolation: bool,
    pub(crate) ends_with: &'a [u8],
    pub(crate) interpolation_started_with_curly_level: usize,

    pub(crate) next_action: NextAction,
}

generate_default_string_literal_impl!(Symbol);

impl<'a> StringLiteralExtend<'a> for Symbol<'a> {
    fn extend(
        &mut self,
        buffer: &mut Buffer<'a>,
        _current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction> {
        let start = buffer.pos();

        handle_eof(buffer, start)?;

        todo!("implement symbol.extend")
    }
}
