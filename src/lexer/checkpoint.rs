use crate::lexer::Lexer;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Checkpoint {
    Real {
        buffer_pos: usize,
        literals_stack_size: usize,

        curly_nest: usize,
        paren_nest: usize,
        brack_nest: usize,
    },
    Dummy,
}

impl Checkpoint {
    pub(crate) fn restore(self, lexer: &mut Lexer) {
        match self {
            Checkpoint::Real {
                buffer_pos,
                literals_stack_size,
                curly_nest,
                paren_nest,
                brack_nest,
            } => {
                lexer.buffer.set_pos(buffer_pos);
                lexer.string_literals.truncate(literals_stack_size);
                lexer.current_token = None;
                lexer.curly_nest = curly_nest;
                lexer.paren_nest = paren_nest;
                lexer.brack_nest = brack_nest;
            }
            Checkpoint::Dummy => return,
        }
    }

    pub(crate) fn real(lexer: &Lexer) -> Self {
        Checkpoint::Real {
            buffer_pos: lexer.buffer.pos(),
            literals_stack_size: lexer.string_literals.size(),
            curly_nest: lexer.curly_nest,
            paren_nest: lexer.paren_nest,
            brack_nest: lexer.brack_nest,
        }
    }
}

#[test]
fn test_checkpoint() {
    use crate::{loc::loc, token::token};

    let mut lexer = Lexer::new(b"1 2 3");

    assert_eq!(lexer.next_token(), token!(tINTEGER, loc!(0, 1)));

    let checkpoint = Checkpoint::real(&lexer);
    assert_eq!(lexer.next_token(), token!(tINTEGER, loc!(2, 3)));

    checkpoint.restore(&mut lexer);
    assert_eq!(lexer.next_token(), token!(tINTEGER, loc!(2, 3)));

    assert_eq!(lexer.next_token(), token!(tINTEGER, loc!(4, 5)));
    assert_eq!(lexer.next_token(), token!(tEOF, loc!(5, 5)));
}
