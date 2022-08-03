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

    #[cfg(test)]
    pub(crate) fn size(&self) -> usize {
        self.stack.len()
    }
}
