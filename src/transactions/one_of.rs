use crate::{
    lexer::Checkpoint,
    transactions::{error::ParseError, result::ParseResult},
};

#[derive(Debug)]
pub(crate) struct OneOf<T> {
    checkpoint: Checkpoint,
    name: &'static str,
    inner: Result<T, Vec<ParseError>>,
}

impl<T> OneOf<T> {
    pub(crate) fn new(name: &'static str, checkpoint: Checkpoint) -> Self {
        Self {
            checkpoint,
            name,
            inner: Err(vec![]),
        }
    }

    pub(crate) fn or_else<F>(mut self, f: F) -> Self
    where
        F: FnOnce() -> ParseResult<T>,
    {
        match &mut self.inner {
            Ok(_) => self,
            Err(errors) => {
                let fallback = f();
                match fallback {
                    Ok(value) => {
                        self.inner = Ok(value);
                    }
                    Err(err) => {
                        errors.push(err);
                        // perform a rollback
                        self.checkpoint.restore()
                    }
                }
                self
            }
        }
    }

    pub(crate) fn unwrap(self) -> ParseResult<T> {
        match self.inner {
            Ok(value) => Ok(value),
            Err(errors) => Err(ParseError::OneOfError {
                name: self.name,
                variants: errors,
            }),
        }
    }

    pub(crate) fn required(mut self) -> Self {
        if let Err(errors) = &mut self.inner {
            errors.iter_mut().for_each(|e| e.make_required());
        }

        self
    }

    // Conditionally removes branches that have less
    // values than the winner.
    //
    // If all branches have the same value this method
    // does nothing.
    pub(crate) fn compact(mut self) -> Self {
        if let Err(errors) = &mut self.inner {
            dbg!(errors.iter().map(|e| e.weight()).collect::<Vec<_>>());
            if let Some(max) = errors.iter().map(|e| e.weight()).max() {
                dbg!(max);
                errors.retain(|e| e.weight() == max)
            }
        }

        self
    }
}
