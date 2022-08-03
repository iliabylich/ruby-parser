use crate::{lexer::Checkpoint, parser::Parser};

impl Parser {
    pub(crate) fn new_checkpoint(&self) -> Checkpoint {
        Checkpoint::new(self.lexer.state)
    }
}
