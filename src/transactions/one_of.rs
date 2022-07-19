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

    pub(crate) fn done(self) -> ParseResult<T> {
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
            errors.iter_mut().for_each(|e| e.into_required())
        }

        self
    }
}
