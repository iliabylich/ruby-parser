use crate::state::State;

use super::StateRef;

#[derive(Debug)]
pub(crate) struct OwnedState {
    inner: Box<State>,
}

impl OwnedState {
    pub(crate) fn new(input: &[u8]) -> Self {
        Self {
            inner: Box::new(State::new(input)),
        }
    }
}

impl OwnedState {
    pub(crate) fn new_ref(&mut self) -> StateRef {
        let state_ref: *mut State = &mut *self.inner;
        StateRef { state_ref }
    }
}
