use crate::{
    lexer::{buffer::BufferWithCursor, strings::stack::StringLiteralStack},
    state::State,
    token::Token,
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) struct StateRef {
    pub(crate) state_ref: *mut State,
}

impl std::fmt::Debug for StateRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_mut())
    }
}

impl StateRef {
    fn as_mut(&self) -> &'static mut State {
        unsafe { self.state_ref.as_mut() }.unwrap()
    }

    pub(crate) fn buffer(&self) -> &'static mut BufferWithCursor {
        &mut self.as_mut().buffer
    }

    pub(crate) fn required_new_expr(&self) -> bool {
        self.as_mut().required_new_expr
    }
    pub(crate) fn required_new_expr_mut(&self) -> &'static mut bool {
        &mut self.as_mut().required_new_expr
    }

    pub(crate) fn string_literals(&self) -> &'static mut StringLiteralStack {
        &mut self.as_mut().string_literals
    }

    pub(crate) fn tokens(&self) -> &'static [Token] {
        &self.as_mut().tokens
    }
    pub(crate) fn tokens_mut(&self) -> &'static mut Vec<Token> {
        &mut self.as_mut().tokens
    }

    pub(crate) fn token_idx(&self) -> usize {
        self.as_mut().token_idx
    }
    pub(crate) fn token_idx_mut(&self) -> &'static mut usize {
        &mut self.as_mut().token_idx
    }

    pub(crate) fn curly_nest(&self) -> usize {
        self.as_mut().curly_nest
    }
    pub(crate) fn curly_nest_mut(&self) -> &'static mut usize {
        &mut self.as_mut().curly_nest
    }

    pub(crate) fn paren_nest(&self) -> usize {
        self.as_mut().paren_nest
    }
    pub(crate) fn paren_nest_mut(&self) -> &'static mut usize {
        &mut self.as_mut().paren_nest
    }

    pub(crate) fn brack_nest(&self) -> usize {
        self.as_mut().brack_nest
    }
    pub(crate) fn brack_nest_mut(&self) -> &'static mut usize {
        &mut self.as_mut().brack_nest
    }
}

pub(crate) trait HasStateRef {
    fn state_ref(&self) -> StateRef;
}
macro_rules! generate_state_ref_delegation {
    ($for:tt) => {
        #[allow(dead_code)]
        impl $for {
            pub(crate) fn buffer(&self) -> &'static mut crate::lexer::buffer::BufferWithCursor {
                self.state_ref().buffer()
            }

            pub(crate) fn required_new_expr(&self) -> bool {
                self.state_ref().required_new_expr()
            }
            pub(crate) fn required_new_expr_mut(&self) -> &'static mut bool {
                self.state_ref().required_new_expr_mut()
            }

            pub(crate) fn string_literals(
                &self,
            ) -> &'static mut crate::lexer::strings::stack::StringLiteralStack {
                self.state_ref().string_literals()
            }

            pub(crate) fn tokens(&self) -> &'static [crate::Token] {
                self.state_ref().tokens()
            }
            pub(crate) fn tokens_mut(&self) -> &'static mut Vec<crate::Token> {
                self.state_ref().tokens_mut()
            }

            pub(crate) fn token_idx(&self) -> usize {
                self.state_ref().token_idx()
            }
            pub(crate) fn token_idx_mut(&self) -> &mut usize {
                self.state_ref().token_idx_mut()
            }

            pub(crate) fn curly_nest(&self) -> usize {
                self.state_ref().curly_nest()
            }
            pub(crate) fn curly_nest_mut(&self) -> &'static mut usize {
                self.state_ref().curly_nest_mut()
            }

            pub(crate) fn paren_nest(&self) -> usize {
                self.state_ref().paren_nest()
            }
            pub(crate) fn paren_nest_mut(&self) -> &'static mut usize {
                self.state_ref().paren_nest_mut()
            }

            pub(crate) fn brack_nest(&self) -> usize {
                self.state_ref().brack_nest()
            }
            pub(crate) fn brack_nest_mut(&self) -> &'static mut usize {
                self.state_ref().brack_nest_mut()
            }
        }
    };
}
pub(crate) use generate_state_ref_delegation;
