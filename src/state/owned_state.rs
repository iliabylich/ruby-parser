use crate::{
    lexer::{buffer::BufferWithCursor, strings::stack::StringLiteralStack},
    state::State,
    token::Token,
};

use super::StateRef;

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

#[allow(dead_code)]
impl OwnedState {
    pub(crate) fn new_ref(&mut self) -> StateRef {
        let state_ref: *mut State = &mut *self.inner;
        StateRef { state_ref }
    }
    pub(crate) fn inner(&self) -> &State {
        &self.inner
    }
    pub(crate) fn inner_mut(&mut self) -> &mut State {
        &mut self.inner
    }

    pub(crate) fn buffer(&self) -> &BufferWithCursor {
        &self.inner().buffer
    }
    pub(crate) fn buffer_mut(&mut self) -> &mut BufferWithCursor {
        &mut self.inner_mut().buffer
    }

    pub(crate) fn required_new_expr(&self) -> bool {
        self.inner().required_new_expr
    }
    pub(crate) fn required_new_expr_mut(&mut self) -> &mut bool {
        &mut self.inner_mut().required_new_expr
    }

    pub(crate) fn string_literals(&self) -> &StringLiteralStack {
        &self.inner().string_literals
    }
    pub(crate) fn string_literals_mut(&mut self) -> &mut StringLiteralStack {
        &mut self.inner_mut().string_literals
    }

    pub(crate) fn curly_nest(&self) -> usize {
        self.inner().curly_nest
    }
    pub(crate) fn curly_nest_mut(&mut self) -> &mut usize {
        &mut self.inner_mut().curly_nest
    }

    pub(crate) fn paren_nest(&self) -> usize {
        self.inner().paren_nest
    }
    pub(crate) fn paren_nest_mut(&mut self) -> &mut usize {
        &mut self.inner_mut().paren_nest
    }

    pub(crate) fn brack_nest(&self) -> usize {
        self.inner().brack_nest
    }
    pub(crate) fn brack_nest_mut(&mut self) -> &mut usize {
        &mut self.inner_mut().brack_nest
    }
}
