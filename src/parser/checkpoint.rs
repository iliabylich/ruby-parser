use crate::{builder::Constructor, lexer::Checkpoint, parser::Parser};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn new_checkpoint(&self) -> Checkpoint {
        Checkpoint::new(self.lexer.state)
    }
}
