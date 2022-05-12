use crate::op_precedence::OpPrecedence;

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
