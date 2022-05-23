pub(crate) mod literal;
pub(crate) mod stack;

use crate::lexer::buffer::Buffer;
use literal::{StringExtendAction, StringLiteral};

pub(crate) fn parse_string<'a>(
    literal: &mut StringLiteral<'a>,
    buffer: &mut Buffer<'a>,
    current_curly_nest: usize,
) -> StringExtendAction<'a> {
    literal.extend(buffer, current_curly_nest)
}

#[cfg(test)]
mod tests;
