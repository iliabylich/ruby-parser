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

#[derive(PartialEq, Eq)]
pub(crate) enum OpPrecedence {
    Left(u8),
    Right(u8),
    Unknown,
}

impl PartialOrd for OpPrecedence {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_number().partial_cmp(&other.as_number())
    }
}

impl PartialEq<u8> for OpPrecedence {
    fn eq(&self, other: &u8) -> bool {
        self.as_number() == *other
    }
}

impl PartialOrd<u8> for OpPrecedence {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.as_number().partial_cmp(other)
    }
}

impl OpPrecedence {
    pub(crate) fn as_number(&self) -> u8 {
        match self {
            OpPrecedence::Left(n) => *n,
            OpPrecedence::Right(n) => *n,
            OpPrecedence::Unknown => 0,
        }
    }

    pub(crate) fn is_right_associative(&self) -> bool {
        matches!(self, Self::Right(_))
    }

    pub(crate) fn is_left_associative(&self) -> bool {
        matches!(self, Self::Left(_))
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
