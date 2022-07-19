use crate::state::StateRef;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Checkpoint {
    buffer_pos: usize,
    literals_stack_size: usize,

    curly_nest: usize,
    paren_nest: usize,
    brack_nest: usize,

    state_ref: StateRef,
}

impl Checkpoint {
    pub(crate) fn new(state_ref: StateRef) -> Self {
        let mut checkpoint = Self {
            buffer_pos: state_ref.buffer().pos(),
            literals_stack_size: state_ref.string_literals().size(),

            curly_nest: state_ref.curly_nest(),
            paren_nest: state_ref.paren_nest(),
            brack_nest: state_ref.brack_nest(),

            state_ref,
        };

        // TODO: get rid of this. instead, introduce a bi-directional conveyor on the parser level
        if let Some(token) = state_ref.current_token() {
            // If there's a cached token
            // remember position _before_ this token
            checkpoint.buffer_pos = token.loc.start;
        }

        checkpoint
    }

    pub(crate) fn restore(&self) {
        self.state_ref.buffer().set_pos(self.buffer_pos);
        self.state_ref
            .string_literals()
            .truncate(self.literals_stack_size);
        *self.state_ref.current_token() = None;
        *self.state_ref.curly_nest_mut() = self.curly_nest;
        *self.state_ref.paren_nest_mut() = self.paren_nest;
        *self.state_ref.brack_nest_mut() = self.brack_nest;
    }
}

#[test]
fn test_checkpoint() {
    use crate::{lexer::Lexer, loc::loc, token::token};

    let (mut lexer, mut state) = Lexer::new_managed(b"1 2 3");
    let state_ref = state.new_ref();

    assert_eq!(lexer.next_token(), token!(tINTEGER, loc!(0, 1)));

    let checkpoint = Checkpoint::new(state_ref);
    assert_eq!(lexer.next_token(), token!(tINTEGER, loc!(2, 3)));

    checkpoint.restore();
    assert_eq!(lexer.next_token(), token!(tINTEGER, loc!(2, 3)));

    assert_eq!(lexer.next_token(), token!(tINTEGER, loc!(4, 5)));
    assert_eq!(lexer.next_token(), token!(tEOF, loc!(5, 5)));
}
