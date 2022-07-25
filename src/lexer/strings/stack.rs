use crate::lexer::strings::literal::StringLiteral;

#[derive(Debug)]
pub(crate) struct StringLiteralStack {
    stack: Vec<StringLiteral>,
}

impl StringLiteralStack {
    pub(crate) fn new() -> Self {
        Self { stack: vec![] }
    }

    pub(crate) fn last(&self) -> Option<&StringLiteral> {
        self.stack.last()
    }

    pub(crate) fn last_mut(&mut self) -> Option<&mut StringLiteral> {
        self.stack.last_mut()
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop().unwrap();
    }

    pub(crate) fn push(&mut self, literal: StringLiteral) {
        self.stack.push(literal);
    }

    #[allow(dead_code)]
    pub(crate) fn size(&self) -> usize {
        self.stack.len()
    }

    #[allow(dead_code)]
    pub(crate) fn truncate(&mut self, new_size: usize) {
        debug_assert!(
            new_size <= self.size(),
            "new size is {}, old is {}",
            new_size,
            self.size()
        );

        self.stack.truncate(new_size);
    }
}
