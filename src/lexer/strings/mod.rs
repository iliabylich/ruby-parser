use std::ops::ControlFlow;

pub(crate) mod action;
pub(crate) mod escapes;
pub(crate) mod handlers;
pub(crate) mod literal;
pub(crate) mod stack;
pub(crate) mod types;

use crate::lexer::buffer::BufferWithCursor;
use action::StringExtendAction;
use literal::{StringLiteral, StringLiteralExtend};

pub(crate) fn parse_string<'a>(
    literal: &mut StringLiteral<'a>,
    buffer: &mut BufferWithCursor<'a>,
    current_curly_nest: usize,
) -> StringExtendAction<'a> {
    match literal.extend(buffer, current_curly_nest) {
        ControlFlow::Continue(_) => unreachable!("literal.extend always return Break(_)"),
        ControlFlow::Break(action) => action,
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_helpers;
