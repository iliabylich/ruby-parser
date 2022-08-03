use crate::state::StateRef;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Checkpoint {
    token_idx: usize,

    state_ref: StateRef,
}

impl Checkpoint {
    pub(crate) fn new(state_ref: StateRef) -> Self {
        Self {
            token_idx: state_ref.token_idx(),

            state_ref,
        }
    }

    pub(crate) fn restore(&self) {
        debug_assert!(
            self.token_idx < self.state_ref.tokens().len(),
            "trying to set token_idx to out-of-bound index (expected {} idx < {} len)",
            self.token_idx,
            self.state_ref.tokens().len()
        );
        self.state_ref.set_token_idx(self.token_idx);
    }
}

#[test]
fn test_checkpoint() {
    use crate::{lexer::Lexer, loc::loc, token::token};

    let (mut lexer, mut state) = Lexer::new_managed(b"1 2 3");
    let state_ref = state.new_ref();

    assert_eq!(lexer.current_token(), token!(tINTEGER, loc!(0, 1)));
    lexer.skip_token();

    let checkpoint = Checkpoint::new(state_ref);
    assert_eq!(lexer.current_token(), token!(tINTEGER, loc!(2, 3)));
    lexer.skip_token();

    checkpoint.restore();
    assert_eq!(lexer.current_token(), token!(tINTEGER, loc!(2, 3)));
    lexer.skip_token();

    assert_eq!(lexer.current_token(), token!(tINTEGER, loc!(4, 5)));
    lexer.skip_token();
    assert_eq!(lexer.current_token(), token!(tEOF, loc!(5, 5)));
    lexer.skip_token();
}
