use crate::{builder::Constructor, lexer::Checkpoint, parser::Parser};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    fn new_checkpoint(&self) -> Checkpoint {
        Checkpoint::real(&self.lexer)
    }

    fn dummy_checkpoint(&self) -> Checkpoint {
        Checkpoint::Dummy
    }

    fn restore_checkpoint(&mut self, checkpoint: Checkpoint) {
        checkpoint.restore(&mut self.lexer)
    }
}
