use crate::{builder::Constructor, lexer::Checkpoint, parser::Parser};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn new_checkpoint(&self) -> Checkpoint {
        Checkpoint::real(&self.lexer)
    }

    pub(crate) fn dummy_checkpoint(&self) -> Checkpoint {
        Checkpoint::Dummy
    }

    pub(crate) fn restore_checkpoint(&mut self, checkpoint: Checkpoint) {
        checkpoint.restore(&mut self.lexer)
    }
}
