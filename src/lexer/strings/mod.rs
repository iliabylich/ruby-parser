pub(crate) mod action;
pub(crate) mod handlers;
pub(crate) mod literal;
pub(crate) mod stack;
pub(crate) mod types;

use crate::lexer::buffer::Buffer;
use action::StringExtendAction;
use literal::{StringLiteral, StringLiteralExtend};

pub(crate) fn parse_string<'a>(
    literal: &mut StringLiteral<'a>,
    buffer: &mut Buffer<'a>,
    current_curly_nest: usize,
) -> StringExtendAction {
    match literal.extend(buffer, current_curly_nest) {
        std::ops::ControlFlow::Continue(_) => unreachable!("literal.extend always return Break(_)"),
        std::ops::ControlFlow::Break(action) => action,
    }
}

#[cfg(test)]
mod tests;
