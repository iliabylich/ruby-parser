use crate::lexer::Lexer;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Checkpoint {
    Real {
        buffer_pos: usize,
        literals_stack_size: usize,
    },
    Dummy,
}

impl Checkpoint {
    pub(crate) fn restore(self, lexer: &mut Lexer) {
        match self {
            Checkpoint::Real {
                buffer_pos,
                literals_stack_size,
            } => {
                lexer.buffer.set_pos(buffer_pos);
                lexer.string_literals.truncate(literals_stack_size);
                lexer.current_token = None;
            }
            Checkpoint::Dummy => return,
        }
    }

    pub(crate) fn real(lexer: &Lexer) -> Self {
        Checkpoint::Real {
            buffer_pos: lexer.buffer.pos(),
            literals_stack_size: lexer.string_literals.size(),
        }
    }
}

#[test]
fn test_checkpoint() {
    use crate::token::token;

    let mut lexer = Lexer::new(b"1 2 3");

    assert_eq!(lexer.next_token(), token!(tINTEGER, 0, 1));

    let checkpoint = Checkpoint::real(&lexer);
    assert_eq!(lexer.next_token(), token!(tINTEGER, 2, 3));

    checkpoint.restore(&mut lexer);
    assert_eq!(lexer.next_token(), token!(tINTEGER, 2, 3));

    assert_eq!(lexer.next_token(), token!(tINTEGER, 4, 5));
    assert_eq!(lexer.next_token(), token!(tEOF, 5, 5));
}
