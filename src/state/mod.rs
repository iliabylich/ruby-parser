use crate::{
    lexer::{buffer::BufferWithCursor, strings::stack::StringLiteralStack},
    token::Token,
};

struct State<'a> {
    pub(crate) buffer: BufferWithCursor<'a>,
    pub(crate) required_new_expr: bool,

    pub(crate) string_literals: StringLiteralStack,

    pub(crate) current_token: Option<Token>,

    pub(crate) curly_nest: usize,
    pub(crate) paren_nest: usize,
    pub(crate) brack_nest: usize,
}

impl<'a> State<'a> {
    fn new(input: &'a [u8]) -> Self {
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

pub(crate) struct OwnedState<'a> {
    inner: Box<State<'a>>,
}

impl<'a> OwnedState<'a> {
    pub(crate) fn new(input: &'a [u8]) -> Self {
        Self {
            inner: Box::new(State::new(input)),
        }
    }
}

pub(crate) struct StateRef<'a> {
    state_ref: *mut State<'a>,
}

impl<'a> From<&mut OwnedState<'a>> for StateRef<'a> {
    fn from(owned: &mut OwnedState<'a>) -> Self {
        let state_ref: *mut State<'a> = owned.inner_mut();
        Self { state_ref }
    }
}

pub(crate) trait StateApi<'a> {
    fn buffer(&self) -> &BufferWithCursor<'a>;
    fn buffer_mut(&mut self) -> &mut BufferWithCursor<'a>;

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

impl<'a> StateApi<'a> for State<'a> {
    fn buffer(&self) -> &BufferWithCursor<'a> {
        &self.buffer
    }
    fn buffer_mut(&mut self) -> &mut BufferWithCursor<'a> {
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
        impl<'a> StateApi<'a> for $for<'a> {
            fn buffer(&self) -> &BufferWithCursor<'a> {
                self.inner().buffer()
            }
            fn buffer_mut(&mut self) -> &mut BufferWithCursor<'a> {
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

trait HoldsState<'a> {
    fn inner(&self) -> &State<'a>;
    fn inner_mut(&mut self) -> &mut State<'a>;
}

impl<'a> HoldsState<'a> for OwnedState<'a> {
    fn inner(&self) -> &State<'a> {
        &self.inner
    }
    fn inner_mut(&mut self) -> &mut State<'a> {
        &mut self.inner
    }
}
generate_impl_delegation!(OwnedState);

impl<'a> HoldsState<'a> for StateRef<'a> {
    fn inner(&self) -> &State<'a> {
        unsafe { self.state_ref.as_ref() }.unwrap()
    }
    fn inner_mut(&mut self) -> &mut State<'a> {
        unsafe { self.state_ref.as_mut() }.unwrap()
    }
}
generate_impl_delegation!(StateRef);
