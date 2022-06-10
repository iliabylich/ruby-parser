use crate::op_precedence::OpPrecedence;
use crate::token::TokenValue;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinOp {
    // %nonassoc, 1
    kIF_MOD,
    kUNLESS_MOD,
    kWHILE_MOD,
    kUNTIL_MOD,
    kIN,

    // %left, 2
    kOR,
    kAND,

    // %right, 3
    kNOT,

    // %nonassoc, 4
    kDEFINED,

    // %right, 5
    tEQL,
    tOP_ASGN,

    // %left, 6
    kRESCUE_MOD,

    // %right, 7
    tEH,
    tCOLON,

    // %nonassoc, 8
    tDOT2,
    tDOT3,
    tBDOT2,
    tBDOT3,

    // %left, 9
    tOROP,

    // %left, 10
    tANDOP,

    // %nonassoc, 11
    tCMP,
    tEQ,
    tEQQ,
    tNEQ,
    tMATCH,
    tNMATCH,

    // %left, 12
    tGT,
    tGEQ,
    tLT,
    tLEQ,

    // %left, 13
    tPIPE,
    tCARET,

    // %left, 14
    tAMPER,

    // %left, 15
    tLSHFT,
    tRSHFT,

    // %left, 16
    tPLUS,
    tMINUS,

    // %left, 17
    tSTAR,
    tDIVIDE,
    tPERCENT,

    // %right, 18
    tUMINUS_NUM,
    tUMINUS,

    // %right, 19
    tPOW,

    // %right, 20
    tBANG,
    tTILDE,
    tUPLUS,
}

impl std::convert::TryFrom<&TokenValue<'_>> for BinOp {
    type Error = ();

    fn try_from(token: &TokenValue) -> Result<Self, Self::Error> {
        match token {
            TokenValue::kIF_MOD => Ok(BinOp::kIF_MOD),
            TokenValue::kUNLESS_MOD => Ok(BinOp::kUNLESS_MOD),
            TokenValue::kWHILE_MOD => Ok(BinOp::kWHILE_MOD),
            TokenValue::kUNTIL_MOD => Ok(BinOp::kUNTIL_MOD),
            TokenValue::kIN => Ok(BinOp::kIN),
            TokenValue::kOR => Ok(BinOp::kOR),
            TokenValue::kAND => Ok(BinOp::kAND),
            TokenValue::kNOT => Ok(BinOp::kNOT),
            TokenValue::kDEFINED => Ok(BinOp::kDEFINED),
            TokenValue::tEQL => Ok(BinOp::tEQL),
            TokenValue::tOP_ASGN => Ok(BinOp::tOP_ASGN),
            TokenValue::kRESCUE_MOD => Ok(BinOp::kRESCUE_MOD),
            TokenValue::tEH => Ok(BinOp::tEH),
            TokenValue::tCOLON => Ok(BinOp::tCOLON),
            TokenValue::tDOT2 => Ok(BinOp::tDOT2),
            TokenValue::tDOT3 => Ok(BinOp::tDOT3),
            TokenValue::tBDOT2 => Ok(BinOp::tBDOT2),
            TokenValue::tBDOT3 => Ok(BinOp::tBDOT3),
            TokenValue::tOROP => Ok(BinOp::tOROP),
            TokenValue::tANDOP => Ok(BinOp::tANDOP),
            TokenValue::tCMP => Ok(BinOp::tCMP),
            TokenValue::tEQ => Ok(BinOp::tEQ),
            TokenValue::tEQQ => Ok(BinOp::tEQQ),
            TokenValue::tNEQ => Ok(BinOp::tNEQ),
            TokenValue::tMATCH => Ok(BinOp::tMATCH),
            TokenValue::tNMATCH => Ok(BinOp::tNMATCH),
            TokenValue::tGT => Ok(BinOp::tGT),
            TokenValue::tGEQ => Ok(BinOp::tGEQ),
            TokenValue::tLT => Ok(BinOp::tLT),
            TokenValue::tLEQ => Ok(BinOp::tLEQ),
            TokenValue::tPIPE => Ok(BinOp::tPIPE),
            TokenValue::tCARET => Ok(BinOp::tCARET),
            TokenValue::tAMPER => Ok(BinOp::tAMPER),
            TokenValue::tLSHFT => Ok(BinOp::tLSHFT),
            TokenValue::tRSHFT => Ok(BinOp::tRSHFT),
            TokenValue::tPLUS => Ok(BinOp::tPLUS),
            TokenValue::tMINUS => Ok(BinOp::tMINUS),
            TokenValue::tSTAR => Ok(BinOp::tSTAR),
            TokenValue::tDIVIDE => Ok(BinOp::tDIVIDE),
            TokenValue::tPERCENT => Ok(BinOp::tPERCENT),
            TokenValue::tUMINUS_NUM => Ok(BinOp::tUMINUS_NUM),
            TokenValue::tUMINUS => Ok(BinOp::tUMINUS),
            TokenValue::tPOW => Ok(BinOp::tPOW),
            TokenValue::tBANG => Ok(BinOp::tBANG),
            TokenValue::tTILDE => Ok(BinOp::tTILDE),
            TokenValue::tUPLUS => Ok(BinOp::tUPLUS),
            _ => Err(()),
        }
    }
}

impl BinOp {
    pub(crate) fn precedence(&self) -> OpPrecedence {
        use BinOp::*;

        match self {
            kIF_MOD | kUNLESS_MOD | kWHILE_MOD | kUNTIL_MOD | kIN => OpPrecedence::None(1),
            kOR | kAND => OpPrecedence::Left(1),
            kNOT => OpPrecedence::Right(3),
            kDEFINED => OpPrecedence::None(4),
            tEQL | tOP_ASGN => OpPrecedence::Right(5),
            kRESCUE_MOD => OpPrecedence::Left(6),
            tEH | tCOLON => OpPrecedence::Right(7),
            tDOT2 | tDOT3 | tBDOT2 | tBDOT3 => OpPrecedence::None(8),
            tOROP => OpPrecedence::Left(9),
            tANDOP => OpPrecedence::Left(10),
            tCMP | tEQ | tEQQ | tNEQ | tMATCH | tNMATCH => OpPrecedence::None(11),
            tGT | tGEQ | tLT | tLEQ => OpPrecedence::Left(12),
            tPIPE | tCARET => OpPrecedence::Left(13),
            tAMPER => OpPrecedence::Left(14),
            tLSHFT | tRSHFT => OpPrecedence::Left(15),
            tPLUS | tMINUS => OpPrecedence::Left(16),
            tSTAR | tDIVIDE | tPERCENT => OpPrecedence::Left(17),
            tUMINUS_NUM | tUMINUS => OpPrecedence::Right(18),
            tPOW => OpPrecedence::Right(19),
            tBANG | tTILDE | tUPLUS => OpPrecedence::Right(20),
        }
    }
}
