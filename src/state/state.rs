use crate::{
    lexer::{buffer::BufferWithCursor, strings::stack::StringLiteralStack},
    token::Token,
};

#[derive(Debug)]
pub(crate) struct State {
    pub(crate) buffer: BufferWithCursor,
    pub(crate) required_new_expr: bool,

    pub(crate) string_literals: StringLiteralStack,

    pub(crate) current_token: Option<Token>,

    pub(crate) curly_nest: usize,
    pub(crate) paren_nest: usize,
    pub(crate) brack_nest: usize,
}

impl State {
    pub(crate) fn new(input: &[u8]) -> Self {
        Self {
            buffer: BufferWithCursor::new(input),
            required_new_expr: false,

            string_literals: StringLiteralStack::new(),

            current_token: None,

            curly_nest: 0,
            paren_nest: 0,
            brack_nest: 0,
        }
    }
}
