use crate::{
    lexer::{buffer::BufferWithCursor, strings::stack::StringLiteralStack},
    state::{OwnedState, State},
    token::Token,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct StateRef {
    state_ref: *mut State,
}

impl From<&mut OwnedState> for StateRef {
    fn from(owned: &mut OwnedState) -> Self {
        let state_ref: *mut State = owned.inner_mut();
        Self { state_ref }
    }
}

impl StateRef {
    fn as_mut(&self) -> &'static mut State {
        unsafe { self.state_ref.as_mut() }.unwrap()
    }

    fn buffer(&self) -> &'static mut BufferWithCursor {
        &mut self.as_mut().buffer
    }

    fn required_new_expr(&self) -> bool {
        self.as_mut().required_new_expr
    }
    fn required_new_expr_mut(&self) -> &'static mut bool {
        &mut self.as_mut().required_new_expr
    }

    fn string_literals(&self) -> &'static mut StringLiteralStack {
        &mut self.as_mut().string_literals
    }

    fn current_token(&self) -> &'static mut Option<Token> {
        &mut self.as_mut().current_token
    }

    fn curly_nest(&self) -> usize {
        self.as_mut().curly_nest
    }
    fn curly_nest_mut(&mut self) -> &'static mut usize {
        &mut self.as_mut().curly_nest
    }

    fn paren_nest(&self) -> usize {
        self.as_mut().paren_nest
    }
    fn paren_nest_mut(&mut self) -> &'static mut usize {
        &mut self.as_mut().paren_nest
    }

    fn brack_nest(&self) -> usize {
        self.as_mut().brack_nest
    }
    fn brack_nest_mut(&mut self) -> &'static mut usize {
        &mut self.as_mut().brack_nest
    }
}

pub(crate) trait HasStateRef {
    fn state_ref(&self) -> StateRef;
}
macro_rules! generate_state_ref_delegation {
    ($for:tt) => {
        impl $for {
            pub(crate) fn buffer(&self) -> &crate::lexer::buffer::BufferWithCursor {
                self.state_ref().buffer()
            }
            pub(crate) fn buffer_mut(&mut self) -> &mut crate::lexer::buffer::BufferWithCursor {
                self.state_ref().buffer_mut()
            }

            pub(crate) fn required_new_expr(&self) -> bool {
                self.state_ref().required_new_expr()
            }
            pub(crate) fn required_new_expr_mut(&mut self) -> &mut bool {
                self.state_ref().required_new_expr_mut()
            }

            pub(crate) fn string_literals(
                &self,
            ) -> &mut crate::lexer::strings::stack::StringLiteralStack {
                self.state_ref().string_literals_mut()
            }

            pub(crate) fn current_token(&self) -> &Option<crate::token::Token> {
                self.state_ref().current_token()
            }
            pub(crate) fn current_token_mut(&mut self) -> &mut Option<crate::token::Token> {
                self.state_ref().current_token_mut()
            }

            pub(crate) fn curly_nest(&self) -> usize {
                self.state_ref().curly_nest()
            }
            pub(crate) fn curly_nest_mut(&mut self) -> &mut usize {
                self.state_ref().curly_nest_mut()
            }

            pub(crate) fn paren_nest(&self) -> usize {
                self.state_ref().paren_nest()
            }
            pub(crate) fn paren_nest_mut(&mut self) -> &mut usize {
                self.state_ref().paren_nest_mut()
            }

            pub(crate) fn brack_nest(&self) -> usize {
                self.state_ref().brack_nest()
            }
            pub(crate) fn brack_nest_mut(&mut self) -> &mut usize {
                self.state_ref().brack_nest_mut()
            }
        }
    };
}
pub(crate) use generate_state_ref_delegation;
