use crate::op_precedence::OpPrecedence;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Token<'a>(pub TokenValue<'a>, pub Loc);

impl<'a> Token<'a> {
    pub fn value(&self) -> TokenValue<'a> {
        self.0
    }

    pub fn loc(&self) -> Loc {
        self.1
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenValue<'a> {
    tINTEGER(&'a str),
    BinOp(BinOp),
    tLPAREN,
    tRPAREN,
    tEOF,

    // TODO: replace with diagnostics
    Error(char),

    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Loc(pub usize, pub usize);

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinOp {
    tPLUS,
    tMINUS,
    tSTAR,
    tDIVIDE,
    tPOW,
}

impl BinOp {
    pub(crate) fn precedence(&self) -> OpPrecedence {
        match self {
            BinOp::tPLUS | BinOp::tMINUS => OpPrecedence::Left(1),
            BinOp::tSTAR | BinOp::tDIVIDE => OpPrecedence::Left(2),
            BinOp::tPOW => OpPrecedence::Right(3),
        }
    }
}
