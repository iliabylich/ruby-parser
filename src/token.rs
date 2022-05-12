use crate::op_precedence::OpPrecedence;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    tINTEGER(u32),
    BinOp(BinOp),
    tLPAREN,
    tRPAREN,
    tEOF,

    // TODO: replace with diagnostics
    Error(char),

    None,
}

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
