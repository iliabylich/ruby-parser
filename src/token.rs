#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Number(u32),
    Plus,
    Minus,
    Mult,
    Div,
    Pow,
    Lparen,
    Rparen,
    EOF,
    Error(char),
    None,
}

pub(crate) enum OpPrecedence {
    Left(u8),
    Right(u8),
    Unknown,
}

impl OpPrecedence {
    pub(crate) fn number(&self) -> u8 {
        match self {
            OpPrecedence::Left(n) => *n,
            OpPrecedence::Right(n) => *n,
            OpPrecedence::Unknown => 0,
        }
    }
}

impl Token {
    pub(crate) fn precedence(&self) -> OpPrecedence {
        match self {
            Token::Plus | Token::Minus => OpPrecedence::Left(1),
            Token::Mult | Token::Div => OpPrecedence::Left(2),
            Token::Pow => OpPrecedence::Right(3),
            _ => OpPrecedence::Unknown,
        }
    }

    pub(crate) fn is_bin_op(&self) -> bool {
        matches!(
            self,
            Token::Plus | Token::Minus | Token::Mult | Token::Div | Token::Pow
        )
    }
}
