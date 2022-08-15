use crate::{lexer::Checkpoint, parser::Parser};

pub(crate) struct NoCheckpoint;
impl NoCheckpoint {
    pub(crate) fn restore(&self) {}
}

impl Parser {
    pub(crate) fn new_checkpoint(&self) -> Checkpoint {
        Checkpoint::new(self.lexer.state)
    }
}
