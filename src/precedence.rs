use crate::token::TokenKind;

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

impl std::convert::TryFrom<TokenKind> for BinOp {
    type Error = ();

    fn try_from(token: TokenKind) -> Result<Self, Self::Error> {
        match token {
            TokenKind::kIF_MOD => Ok(BinOp::kIF_MOD),
            TokenKind::kUNLESS_MOD => Ok(BinOp::kUNLESS_MOD),
            TokenKind::kWHILE_MOD => Ok(BinOp::kWHILE_MOD),
            TokenKind::kUNTIL_MOD => Ok(BinOp::kUNTIL_MOD),
            TokenKind::kIN => Ok(BinOp::kIN),
            TokenKind::kOR => Ok(BinOp::kOR),
            TokenKind::kAND => Ok(BinOp::kAND),
            TokenKind::kNOT => Ok(BinOp::kNOT),
            TokenKind::kDEFINED => Ok(BinOp::kDEFINED),
            TokenKind::tEQL => Ok(BinOp::tEQL),
            TokenKind::tOP_ASGN => Ok(BinOp::tOP_ASGN),
            TokenKind::kRESCUE_MOD => Ok(BinOp::kRESCUE_MOD),
            TokenKind::tEH => Ok(BinOp::tEH),
            TokenKind::tCOLON => Ok(BinOp::tCOLON),
            TokenKind::tDOT2 => Ok(BinOp::tDOT2),
            TokenKind::tDOT3 => Ok(BinOp::tDOT3),
            TokenKind::tBDOT2 => Ok(BinOp::tBDOT2),
            TokenKind::tBDOT3 => Ok(BinOp::tBDOT3),
            TokenKind::tOROP => Ok(BinOp::tOROP),
            TokenKind::tANDOP => Ok(BinOp::tANDOP),
            TokenKind::tCMP => Ok(BinOp::tCMP),
            TokenKind::tEQ => Ok(BinOp::tEQ),
            TokenKind::tEQQ => Ok(BinOp::tEQQ),
            TokenKind::tNEQ => Ok(BinOp::tNEQ),
            TokenKind::tMATCH => Ok(BinOp::tMATCH),
            TokenKind::tNMATCH => Ok(BinOp::tNMATCH),
            TokenKind::tGT => Ok(BinOp::tGT),
            TokenKind::tGEQ => Ok(BinOp::tGEQ),
            TokenKind::tLT => Ok(BinOp::tLT),
            TokenKind::tLEQ => Ok(BinOp::tLEQ),
            TokenKind::tPIPE => Ok(BinOp::tPIPE),
            TokenKind::tCARET => Ok(BinOp::tCARET),
            TokenKind::tAMPER => Ok(BinOp::tAMPER),
            TokenKind::tLSHFT => Ok(BinOp::tLSHFT),
            TokenKind::tRSHFT => Ok(BinOp::tRSHFT),
            TokenKind::tPLUS => Ok(BinOp::tPLUS),
            TokenKind::tMINUS => Ok(BinOp::tMINUS),
            TokenKind::tSTAR => Ok(BinOp::tSTAR),
            TokenKind::tDIVIDE => Ok(BinOp::tDIVIDE),
            TokenKind::tPERCENT => Ok(BinOp::tPERCENT),
            TokenKind::tUMINUS_NUM => Ok(BinOp::tUMINUS_NUM),
            TokenKind::tUMINUS => Ok(BinOp::tUMINUS),
            TokenKind::tPOW => Ok(BinOp::tPOW),
            TokenKind::tBANG => Ok(BinOp::tBANG),
            TokenKind::tTILDE => Ok(BinOp::tTILDE),
            TokenKind::tUPLUS => Ok(BinOp::tUPLUS),
            _ => Err(()),
        }
    }
}

macro_rules! left_assoc {
    ($n:expr) => {
        Some(($n * 2, $n * 2 + 1))
    };
}

macro_rules! right_assoc {
    ($n:expr) => {
        Some(($n * 2 + 1, $n * 2))
    };
}

macro_rules! non_assoc {
    ($n:expr) => {
        left_assoc!($n)
    };
}

impl TokenKind {
    pub(crate) fn precedence(&self) -> Option<(u8, u8)> {
        use TokenKind::*;

        match self {
            kIF_MOD | kUNLESS_MOD | kWHILE_MOD | kUNTIL_MOD | kIN => non_assoc!(1),
            kOR | kAND => left_assoc!(2),
            kNOT => right_assoc!(3),
            kDEFINED => non_assoc!(4),
            tEQL | tOP_ASGN => right_assoc!(5),
            kRESCUE_MOD => left_assoc!(6),
            tEH | tCOLON => right_assoc!(7),
            tDOT2 | tDOT3 | tBDOT2 | tBDOT3 => non_assoc!(8),
            tOROP => left_assoc!(9),
            tANDOP => left_assoc!(10),
            tCMP | tEQ | tEQQ | tNEQ | tMATCH | tNMATCH => non_assoc!(11),
            tGT | tGEQ | tLT | tLEQ => left_assoc!(12),
            tPIPE | tCARET => left_assoc!(13),
            tAMPER => left_assoc!(14),
            tLSHFT | tRSHFT => left_assoc!(15),
            tPLUS | tMINUS => left_assoc!(16),
            tSTAR | tDIVIDE | tPERCENT => left_assoc!(17),
            tUMINUS_NUM | tUMINUS => right_assoc!(18),
            tPOW => right_assoc!(19),
            tBANG | tTILDE | tUPLUS => right_assoc!(20),
            _ => None,
        }
    }
}
