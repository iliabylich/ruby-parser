use crate::lexer::strings::literal::StringLiteral;

#[derive(Debug)]
pub(crate) struct StringLiteralStack<'a> {
    stack: Vec<StringLiteral<'a>>,
}

impl<'a> StringLiteralStack<'a> {
    pub(crate) fn new() -> Self {
        Self { stack: vec![] }
    }

    pub(crate) fn last(&self) -> Option<StringLiteral<'a>> {
        self.stack.last().map(|literal| *literal)
    }

    pub(crate) fn last_mut(&mut self) -> Option<&mut StringLiteral<'a>> {
        self.stack.last_mut()
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop().unwrap();
    }

    pub(crate) fn push(&mut self, literal: StringLiteral<'a>) {
        self.stack.push(literal);
    }

    #[cfg(test)]
    pub(crate) fn size(&self) -> usize {
        self.stack.len()
    }
}
