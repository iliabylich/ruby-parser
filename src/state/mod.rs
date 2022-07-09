use crate::{
    lexer::{buffer::BufferWithCursor, strings::stack::StringLiteralStack},
    token::Token,
};

struct State {
    pub(crate) buffer: BufferWithCursor,
    pub(crate) required_new_expr: bool,

    pub(crate) string_literals: StringLiteralStack,

    pub(crate) current_token: Option<Token>,

    pub(crate) curly_nest: usize,
    pub(crate) paren_nest: usize,
    pub(crate) brack_nest: usize,
}

impl State {
    fn new(input: &[u8]) -> Self {
        Self {
            buffer: BufferWithCursor::new(input),
            required_new_expr: false,
            string_literals: StringLiteralStack::new(),
            current_token: None,
            curly_nest: 0,
            paren_nest: 0,
            brack_nest: 0,
        }
    }
}

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

pub(crate) struct StateRef {
    state_ref: *mut State,
}

impl From<&mut OwnedState> for StateRef {
    fn from(owned: &mut OwnedState) -> Self {
        let state_ref: *mut State = owned.inner_mut();
        Self { state_ref }
    }
}

pub(crate) trait StateApi {
    fn buffer(&self) -> &BufferWithCursor;
    fn buffer_mut(&mut self) -> &mut BufferWithCursor;

    fn required_new_expr(&self) -> bool;
    fn required_new_expr_mut(&mut self) -> &mut bool;

    fn string_literals(&self) -> &StringLiteralStack;
    fn string_literals_mut(&mut self) -> &mut StringLiteralStack;

    fn current_token(&self) -> &Option<Token>;
    fn current_token_mut(&mut self) -> &mut Option<Token>;

    fn curly_nest(&self) -> usize;
    fn curly_nest_mut(&mut self) -> &mut usize;

    fn paren_nest(&self) -> usize;
    fn paren_nest_mut(&mut self) -> &mut usize;

    fn brack_nest(&self) -> usize;
    fn brack_nest_mut(&mut self) -> &mut usize;
}

impl StateApi for State {
    fn buffer(&self) -> &BufferWithCursor {
        &self.buffer
    }
    fn buffer_mut(&mut self) -> &mut BufferWithCursor {
        &mut self.buffer
    }

    fn required_new_expr(&self) -> bool {
        self.required_new_expr
    }
    fn required_new_expr_mut(&mut self) -> &mut bool {
        &mut self.required_new_expr
    }

    fn string_literals(&self) -> &StringLiteralStack {
        &self.string_literals
    }
    fn string_literals_mut(&mut self) -> &mut StringLiteralStack {
        &mut self.string_literals
    }

    fn current_token(&self) -> &Option<Token> {
        &self.current_token
    }
    fn current_token_mut(&mut self) -> &mut Option<Token> {
        &mut self.current_token
    }

    fn curly_nest(&self) -> usize {
        self.curly_nest
    }
    fn curly_nest_mut(&mut self) -> &mut usize {
        &mut self.curly_nest
    }

    fn paren_nest(&self) -> usize {
        self.paren_nest
    }
    fn paren_nest_mut(&mut self) -> &mut usize {
        &mut self.paren_nest
    }

    fn brack_nest(&self) -> usize {
        self.brack_nest
    }
    fn brack_nest_mut(&mut self) -> &mut usize {
        &mut self.brack_nest
    }
}

macro_rules! generate_impl_delegation {
    ($for:tt) => {
        impl StateApi for $for {
            fn buffer(&self) -> &BufferWithCursor {
                self.inner().buffer()
            }
            fn buffer_mut(&mut self) -> &mut BufferWithCursor {
                self.inner_mut().buffer_mut()
            }

            fn required_new_expr(&self) -> bool {
                self.inner().required_new_expr()
            }
            fn required_new_expr_mut(&mut self) -> &mut bool {
                self.inner_mut().required_new_expr_mut()
            }

            fn string_literals(&self) -> &StringLiteralStack {
                self.inner().string_literals()
            }
            fn string_literals_mut(&mut self) -> &mut StringLiteralStack {
                self.inner_mut().string_literals_mut()
            }

            fn current_token(&self) -> &Option<Token> {
                self.inner().current_token()
            }
            fn current_token_mut(&mut self) -> &mut Option<Token> {
                self.inner_mut().current_token_mut()
            }

            fn curly_nest(&self) -> usize {
                self.inner().curly_nest()
            }
            fn curly_nest_mut(&mut self) -> &mut usize {
                self.inner_mut().curly_nest_mut()
            }

            fn paren_nest(&self) -> usize {
                self.inner().paren_nest()
            }
            fn paren_nest_mut(&mut self) -> &mut usize {
                self.inner_mut().paren_nest_mut()
            }

            fn brack_nest(&self) -> usize {
                self.inner().brack_nest()
            }
            fn brack_nest_mut(&mut self) -> &mut usize {
                self.inner_mut().brack_nest_mut()
            }
        }
    };
}

trait HoldsState {
    fn inner(&self) -> &State;
    fn inner_mut(&mut self) -> &mut State;
}

impl HoldsState for OwnedState {
    fn inner(&self) -> &State {
        &self.inner
    }
    fn inner_mut(&mut self) -> &mut State {
        &mut self.inner
    }
}
generate_impl_delegation!(OwnedState);

impl HoldsState for StateRef {
    fn inner(&self) -> &State {
        unsafe { self.state_ref.as_ref() }.unwrap()
    }
    fn inner_mut(&mut self) -> &mut State {
        unsafe { self.state_ref.as_mut() }.unwrap()
    }
}
generate_impl_delegation!(StateRef);
