use crate::lexer::buffer::Buffer;

pub(crate) struct StringLiterals<'a> {
    stack: Vec<StringLiteral<'a>>,
}

#[derive(Clone, Copy)]
pub(crate) enum StringLiteral<'a> {
    Plain {
        supports_interpolation: bool,
        currently_in_interpolation: bool,
        ends_with: &'a str,
        interpolation_started_with_curly_level: usize,
    },

    Heredoc {
        supports_interpolation: bool,
        currently_in_interpolation: bool,
        ends_with: &'a str,
        heredoc_id_ended_at: usize,
        interpolation_started_with_curly_level: usize,
    },
}

pub(crate) enum StringLiteralAction<'a> {
    InInterpolation {
        interpolation_started_with_curly_level: usize,
    },
    EmitStringContent {
        content: &'a [u8],
        start: usize,
        end: usize,
    },
    CloseLiteral {
        content: &'a [u8],
        start: usize,
        end: usize,
        jump_to: usize,
    },
}

impl<'a> StringLiterals<'a> {
    pub(crate) fn new() -> Self {
        Self { stack: vec![] }
    }

    pub(crate) fn last(&self) -> Option<StringLiteral<'a>> {
        self.stack.last().map(|literal| *literal)
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop().unwrap();
    }
}

impl<'a> StringLiteral<'a> {
    pub(crate) fn lex(&self, buffer: &mut Buffer) -> StringLiteralAction<'a> {
        todo!()
    }
}
